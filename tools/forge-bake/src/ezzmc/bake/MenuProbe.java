package ezzmc.bake;

import com.mojang.blaze3d.pipeline.TextureTarget;
import com.mojang.blaze3d.platform.Lighting;
import com.mojang.blaze3d.platform.NativeImage;
import com.mojang.blaze3d.systems.RenderSystem;
import com.mojang.blaze3d.vertex.PoseStack;
import com.mojang.blaze3d.vertex.VertexSorting;
import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.GuiGraphics;
import net.minecraft.client.gui.screens.inventory.AbstractContainerScreen;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.locale.Language;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.Difficulty;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.player.Inventory;
import net.minecraft.world.inventory.AbstractContainerMenu;
import net.minecraft.world.inventory.Slot;
import net.minecraft.world.level.GameRules;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.LevelSettings;
import net.minecraft.world.level.WorldDataConfiguration;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.levelgen.WorldOptions;
import net.minecraft.world.level.levelgen.presets.WorldPresets;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.Vec3;
import org.joml.Matrix4f;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/**
 * v3 (TODO-513): схемы модовых меню. Дампер заходит в плоский творческий мир, ставит каждый модовый
 * блок с блок-сущностью, жмёт по нему ПКМ от лица игрока на сервере (как настоящий клик) и, если
 * открылся контейнерный экран, снимает `menu.slots` и картинку окна (GUI-масштаб 1, без предметов —
 * машина новая, инвентарь пуст). Полупрозрачное затемнение фона отрезается по альфе.
 */
final class MenuProbe {
    static final String WORLD = "ezzmc-bake";
    /** Виртуальный экран GUI-масштаба 1 (ванильные окна ≤ 256, AE2-терминалы подстраиваются под высоту). */
    static final int W = 960;
    static final int H = 540;
    static final int WAIT_TICKS = 8;
    /** Вкладки модов за краем окна (Thermal: ~26 px) — запас рамки снимка. */
    static final int MARGIN = 48;

    record Menu(String name, int imgW, int imgH, int dx, int dy, int bw, int bh, byte[] png,
                List<int[]> slots, String block) {
    }

    final Minecraft mc;
    /** Уже снятые меню (файлы menus/*.bin, в том числе из прошлых запусков). */
    final List<java.nio.file.Path> saved = new ArrayList<>();
    final java.util.Set<String> names = new java.util.HashSet<>();
    /** Сервер открыл меню после клика (иначе — сразу следующий блок). */
    volatile boolean opened;
    /** Шаг 1 (блок поставлен на сервере) и шаг 2 (клик отправлен). */
    volatile boolean placed;
    volatile boolean clicked;
    int placeWait;
    final List<Block> queue = new ArrayList<>();
    int qi = -1;
    int wait;
    int warm;
    volatile boolean used;
    /** Экран, перехваченный в ScreenEvent.Opening (в игру не попал). */
    AbstractContainerScreen<?> pending;
    /** TODO-519: экран мода падает при рендере, пока блок-сущность не досинхронизировалась —
     * повторяем снимок до RENDER_RETRIES тиков. */
    int renderTries;
    static final int RENDER_RETRIES = 20;
    int tried;
    int failed;

    /** Папка состояния обхода (переживает краши модов): base.bin, progress.txt, menus/. */
    static java.nio.file.Path state() throws java.io.IOException {
        java.nio.file.Path d = java.nio.file.Path.of(System.getProperty("ezzmc.bake.state",
                System.getProperty("ezzmc.bake.out") + ".state"));
        java.nio.file.Files.createDirectories(d.resolve("menus"));
        return d;
    }

    MenuProbe(Minecraft mc) throws java.io.IOException {
        this.mc = mc;
        // продолжение после краша: блок из progress.txt уронил игру — начинаем со следующего
        java.nio.file.Path prog = state().resolve("progress.txt");
        if (java.nio.file.Files.exists(prog)) {
            qi = Integer.parseInt(java.nio.file.Files.readString(prog).trim());
            used = true; // блок qi «закончен» (уронил игру) — первый tick сразу берёт следующий
            System.out.println("[ezzmc-bake] resume after block #" + qi);
        }
        try (var files = java.nio.file.Files.list(state().resolve("menus"))) {
            for (java.nio.file.Path f : (Iterable<java.nio.file.Path>) files::iterator) {
                saved.add(f);
                String n = f.getFileName().toString();
                names.add(n.substring(0, n.length() - 4).replace('~', ':').replace('!', '/'));
            }
        }
        for (Block b : BuiltInRegistries.BLOCK) {
            ResourceLocation k = BuiltInRegistries.BLOCK.getKey(b);
            try {
                if (!"minecraft".equals(k.getNamespace()) && b.defaultBlockState().hasBlockEntity()) {
                    queue.add(b);
                }
            } catch (Throwable ignored) {
                // блок без нормального состояния — пропуск
            }
        }
        System.out.println("[ezzmc-bake] menu probe: " + queue.size() + " blocks with BE");
    }

    void createWorld() {
        LevelSettings ls = new LevelSettings(WORLD, GameType.CREATIVE, false, Difficulty.PEACEFUL, true,
                new GameRules(), WorldDataConfiguration.DEFAULT);
        mc.createWorldOpenFlows().createFreshLevel(WORLD, ls, new WorldOptions(0L, false, false),
                ra -> ra.registryOrThrow(Registries.WORLD_PRESET).getHolderOrThrow(WorldPresets.FLAT)
                        .value().createWorldDimensions());
    }

    boolean ready() {
        if (mc.player == null || mc.level == null || mc.getSingleplayerServer() == null) {
            return false;
        }
        if (mc.screen != null) {
            mc.setScreen(null);
        }
        return ++warm > 60;
    }

    BlockPos pos() {
        return mc.player.blockPosition().offset(2, 0, 0);
    }

    /** Один шаг обхода; true — обход закончен. */
    boolean tick() {
        if (qi >= 0) {
            if (pending != null) {
                AbstractContainerScreen<?> scr = pending;
                pending = null;
                try {
                    if (!capture(scr)) {
                        pending = scr; // повтор на следующем тике, экран не закрываем
                        return false;
                    }
                } catch (Throwable t) {
                    System.out.println("[ezzmc-bake] capture failed: " + t);
                }
                close();
                return next();
            }
            // шаг 2: блок с BE дошёл до клиента (или 6 тиков) — теперь клик (иначе меню мода
            // создаётся на клиенте без BE: NPE в экране / экран не создаётся)
            if (placed && !clicked) {
                BlockPos p = pos();
                if (mc.level.getBlockEntity(p) != null || ++placeWait > 6) {
                    clicked = true;
                    click(p);
                }
                return false;
            }
            if (used && (!opened || ++wait > WAIT_TICKS)) {
                failed++;
                close();
                return next();
            }
            return false;
        }
        return next();
    }

    void close() {
        if (mc.player != null && mc.player.containerMenu != mc.player.inventoryMenu) {
            mc.player.closeContainer();
        }
        if (mc.screen != null) {
            mc.setScreen(null);
        }
    }

    boolean next() {
        qi++;
        wait = 0;
        used = false;
        pending = null;
        renderTries = 0;
        opened = false;
        placed = false;
        clicked = false;
        placeWait = 0;
        if (qi >= queue.size()) {
            System.out.println("[ezzmc-bake] menus=" + saved.size() + " tried=" + tried
                    + " no_screen=" + failed);
            return true;
        }
        if (qi % 200 == 0) {
            System.out.println("[ezzmc-bake] menu probe " + qi + "/" + queue.size()
                    + " menus=" + saved.size());
        }
        try {
            java.nio.file.Files.writeString(state().resolve("progress.txt"), Integer.toString(qi));
        } catch (java.io.IOException ignored) {
            // без прогресса продолжение после краша начнётся раньше — не страшно
        }
        Block b = queue.get(qi);
        BlockPos p = pos();
        ServerPlayer sp = mc.getSingleplayerServer().getPlayerList().getPlayer(mc.player.getUUID());
        mc.getSingleplayerServer().execute(() -> {
            try {
                ServerLevel lvl = sp.serverLevel();
                // меню прошлого блока могло остаться открытым на сервере (экран у клиента не создался)
                if (sp.containerMenu != sp.inventoryMenu) {
                    sp.closeContainer();
                }
                // моды выдают стартовые предметы (книги) — на снимке инвентарь должен быть пуст
                sp.getInventory().clearContent();
                lvl.setBlock(p, Blocks.AIR.defaultBlockState(), 3);
                lvl.setBlock(p, b.defaultBlockState(), 3);
                tried++;
            } catch (Throwable t) {
                // мод упал на установке — меню без схемы
                clicked = true;
                used = true;
            }
            placed = true;
        });
        return false;
    }

    /** Шаг 2: ПКМ по уже синхронизированному блоку от лица игрока (на сервере). */
    void click(BlockPos p) {
        ServerPlayer sp = mc.getSingleplayerServer().getPlayerList().getPlayer(mc.player.getUUID());
        mc.getSingleplayerServer().execute(() -> {
            try {
                ServerLevel lvl = sp.serverLevel();
                lvl.getBlockState(p).use(lvl, sp, InteractionHand.MAIN_HAND,
                        new BlockHitResult(Vec3.atCenterOf(p), Direction.UP, p, false));
                opened = sp.containerMenu != sp.inventoryMenu;
                if (!opened) {
                    lvl.setBlock(p, Blocks.AIR.defaultBlockState(), 3);
                }
            } catch (Throwable t) {
                // мод упал на клике — меню без схемы
            }
            used = true;
        });
    }

    /** false — рендер упал, стоит повторить на следующем тике. */
    boolean capture(AbstractContainerScreen<?> scr) {
        AbstractContainerMenu menu = scr.getMenu();
        String name;
        try {
            name = BuiltInRegistries.MENU.getKey(menu.getType()).toString();
        } catch (Throwable t) {
            return true;
        }
        if (names.contains(name) || name.startsWith("minecraft:")) {
            return true;
        }
        String block = BuiltInRegistries.BLOCK.getKey(queue.get(qi)).toString();
        List<int[]> slots = new ArrayList<>();
        for (Slot s : menu.slots) {
            int flags = (s.isActive() ? 1 : 0) | (s.container instanceof Inventory ? 2 : 0);
            slots.add(new int[]{s.x, s.y, flags});
        }
        Menu m;
        try {
            m = render(scr, name, slots, block);
        } catch (Throwable t) {
            if (++renderTries < RENDER_RETRIES) {
                return false;
            }
            System.out.println("[ezzmc-bake] render failed " + name + ": " + t);
            m = new Menu(name, scr.getXSize(), scr.getYSize(), 0, 0, 0, 0, new byte[0], slots, block);
        }
        names.add(name);
        try {
            EzzmcBake.Le w = new EzzmcBake.Le();
            writeMenu(w, m);
            java.nio.file.Path f = state().resolve("menus")
                    .resolve(name.replace(':', '~').replace('/', '!') + ".bin");
            java.nio.file.Files.write(f, w.toBytes());
            saved.add(f);
        } catch (java.io.IOException e) {
            System.out.println("[ezzmc-bake] save failed " + name + ": " + e);
        }
        return true;
    }

    Menu render(AbstractContainerScreen<?> scr, String name, List<int[]> slots, String block)
            throws java.io.IOException {
        scr.init(mc, W, H);
        TextureTarget target = new TextureTarget(W, H, true, Minecraft.ON_OSX);
        target.setClearColor(0, 0, 0, 0);
        target.clear(Minecraft.ON_OSX);
        RenderSystem.backupProjectionMatrix();
        PoseStack mv = RenderSystem.getModelViewStack();
        mv.pushPose();
        mv.setIdentity();
        mv.translate(0, 0, -11000);
        RenderSystem.applyModelViewMatrix();
        RenderSystem.setProjectionMatrix(new Matrix4f().setOrtho(0, W, H, 0, 1000, 21000),
                VertexSorting.ORTHOGRAPHIC_Z);
        byte[] png;
        int x0 = W, y0 = H, x1 = -1, y1 = -1;
        try {
            target.bindWrite(true);
            Lighting.setupFor3DItems();
            GuiGraphics g = new GuiGraphics(mc, mc.renderBuffers().bufferSource());
            scr.render(g, -10000, -10000, 0f);
            g.flush();
            target.unbindWrite();
            try (NativeImage img = new NativeImage(W, H, false)) {
                RenderSystem.bindTexture(target.getColorTextureId());
                img.downloadTexture(0, false);
                img.flipY();
                // затемнение фона (renderBackground) полупрозрачно, окно — непрозрачно; рамку ищем
                // только рядом с окном (±MARGIN): кнопки модов в углу экрана (FTB и т. п.) — не часть окна
                int gl = scr.getGuiLeft(), gt = scr.getGuiTop();
                for (int y = 0; y < H; y++) {
                    for (int x = 0; x < W; x++) {
                        int a = (img.getPixelRGBA(x, y) >>> 24) & 0xFF;
                        boolean near = x >= gl - MARGIN && x < gl + scr.getXSize() + MARGIN
                                && y >= gt - MARGIN && y < gt + scr.getYSize() + MARGIN;
                        if (a < 250 || !near) {
                            img.setPixelRGBA(x, y, 0);
                        } else {
                            x0 = Math.min(x0, x);
                            y0 = Math.min(y0, y);
                            x1 = Math.max(x1, x);
                            y1 = Math.max(y1, y);
                        }
                    }
                }
                if (x1 < 0) {
                    return new Menu(name, scr.getXSize(), scr.getYSize(), 0, 0, 0, 0, new byte[0], slots,
                            block);
                }
                try (NativeImage crop = new NativeImage(x1 - x0 + 1, y1 - y0 + 1, false)) {
                    img.copyRect(crop, x0, y0, 0, 0, x1 - x0 + 1, y1 - y0 + 1, false, false);
                    png = crop.asByteArray();
                }
            }
        } finally {
            mv.popPose();
            RenderSystem.applyModelViewMatrix();
            RenderSystem.restoreProjectionMatrix();
            target.destroyBuffers();
            mc.getMainRenderTarget().bindWrite(true);
        }
        return new Menu(name, scr.getXSize(), scr.getYSize(), x0 - scr.getGuiLeft(), y0 - scr.getGuiTop(),
                x1 - x0 + 1, y1 - y0 + 1, png, slots, block);
    }

    /** u32 n { str key; str value } — все переводы текущего языка игры (ваниль + моды). */
    void writeLang(EzzmcBake.Le w) {
        Map<String, String> all = new LinkedHashMap<>();
        Object lang = Language.getInstance();
        for (Class<?> c = lang.getClass(); c != null; c = c.getSuperclass()) {
            for (Field f : c.getDeclaredFields()) {
                if (Map.class.isAssignableFrom(f.getType())) {
                    try {
                        f.setAccessible(true);
                        Map<?, ?> m = (Map<?, ?>) f.get(lang);
                        for (var e : m.entrySet()) {
                            if (e.getKey() instanceof String k && e.getValue() instanceof String v) {
                                all.put(k, v);
                            }
                        }
                    } catch (Throwable ignored) {
                        // поле недоступно — пропуск
                    }
                }
            }
        }
        w.u32(all.size());
        for (var e : all.entrySet()) {
            w.str(e.getKey());
            w.str(e.getValue());
        }
        System.out.println("[ezzmc-bake] lang entries=" + all.size());
    }

    /**
     * u32 n { str name; str block; u16 img_w, img_h; i16 bg_dx, bg_dy; u16 bg_w, bg_h; u32 png_len; png;
     * u16 nslots { i16 x, y; u8 flags(1 active, 2 player_inv) } }.
     */
    void writeMenus(EzzmcBake.Le w) throws java.io.IOException {
        w.u32(saved.size());
        for (java.nio.file.Path f : saved) {
            w.bytes(java.nio.file.Files.readAllBytes(f));
        }
    }

    static void writeMenu(EzzmcBake.Le w, Menu m) {
        w.str(m.name());
        w.str(m.block());
        w.u16(m.imgW());
        w.u16(m.imgH());
        w.u16(m.dx());
        w.u16(m.dy());
        w.u16(m.bw());
        w.u16(m.bh());
        w.u32(m.png().length);
        w.bytes(m.png());
        w.u16(m.slots().size());
        for (int[] s : m.slots()) {
            w.u16(s[0]);
            w.u16(s[1]);
            w.u8(s[2]);
        }
    }
}
