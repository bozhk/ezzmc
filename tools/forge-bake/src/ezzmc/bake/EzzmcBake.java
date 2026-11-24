package ezzmc.bake;

import com.mojang.blaze3d.pipeline.TextureTarget;
import com.mojang.blaze3d.platform.Lighting;
import com.mojang.blaze3d.platform.NativeImage;
import com.mojang.blaze3d.vertex.PoseStack;
import com.mojang.blaze3d.vertex.VertexSorting;
import com.mojang.blaze3d.systems.RenderSystem;
import net.minecraft.client.Minecraft;
import net.minecraft.client.color.block.BlockColors;
import net.minecraft.client.gui.GuiGraphics;
import net.minecraft.client.gui.screens.TitleScreen;
import net.minecraft.client.renderer.RenderType;
import net.minecraft.client.renderer.block.model.BakedQuad;
import net.minecraft.client.renderer.texture.TextureAtlas;
import net.minecraft.client.renderer.texture.TextureAtlasSprite;
import net.minecraft.client.resources.model.BakedModel;
import net.minecraft.client.resources.model.WeightedBakedModel;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.util.RandomSource;
import net.minecraft.util.random.WeightedEntry;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.EmptyBlockGetter;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.block.state.properties.Property;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.shapes.VoxelShape;
import net.minecraftforge.client.model.data.ModelData;
import net.minecraftforge.common.MinecraftForge;
import net.minecraftforge.event.TickEvent;
import net.minecraftforge.fml.common.Mod;
import org.joml.Matrix4f;
import org.lwjgl.opengl.GL11;

import java.io.IOException;
import java.lang.reflect.Field;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Дампер ezzmc (DEC-018, TODO-502): после загрузки ресурсов пишет все состояния блоков, их
 * запечённые квады, формы, свет, тинты и атлас блоков в один файл `.ezbake`, затем закрывает игру.
 * Активен только при -Dezzmc.bake.out=<путь>. Формат: docs/ref/forge-bake.md.
 */
@Mod("ezzmc_bake")
public final class EzzmcBake {
    static final int VERSION = 3;
    /** 0 — ждём TitleScreen, 1 — мир грузится, 2 — обход машин, 3 — готово. */
    private int phase;
    private byte[] base;
    private MenuProbe probe;

    public EzzmcBake() {
        if (System.getProperty("ezzmc.bake.out") == null) {
            return;
        }
        MinecraftForge.EVENT_BUS.addListener(this::onTick);
        MinecraftForge.EVENT_BUS.addListener(this::onScreenOpen);
    }

    /**
     * Во время обхода машин экраны не открываются в игре: контейнерный забирает дампер (рисует сам,
     * в try/catch), прочие отбрасываются. Иначе окно мода (или чужой оверлей Render.Post), падающее
     * на «голой» машине, роняет весь клиент в обычном кадре.
     */
    private void onScreenOpen(net.minecraftforge.client.event.ScreenEvent.Opening e) {
        if (phase != 2 || e.getNewScreen() == null) {
            return;
        }
        if (e.getNewScreen() instanceof net.minecraft.client.gui.screens.inventory.AbstractContainerScreen<?> s) {
            probe.pending = s;
        }
        e.setCanceled(true);
    }

    private void onTick(TickEvent.ClientTickEvent e) {
        Minecraft mc = Minecraft.getInstance();
        if (!TickEvent.Phase.END.equals(e.phase) || java.util.Objects.nonNull(mc.getOverlay())) {
            return;
        }
        try {
            switch (phase) {
                case 0 -> {
                    if (!(mc.screen instanceof TitleScreen)) {
                        return;
                    }
                    // База (блоки/иконки) печётся один раз; после краша мода — из файла состояния.
                    Path baseFile = MenuProbe.state().resolve("base.bin");
                    if (Files.exists(baseFile)) {
                        base = Files.readAllBytes(baseFile);
                        System.out.println("[ezzmc-bake] resume: base from " + baseFile);
                    } else {
                        long t = System.currentTimeMillis();
                        base = new Dumper(mc).dump();
                        Files.write(baseFile, base);
                        System.out.println("[ezzmc-bake] base " + base.length + " bytes, "
                                + (System.currentTimeMillis() - t) + " ms");
                    }
                    probe = new MenuProbe(mc);
                    phase = 1;
                    if (Boolean.getBoolean("ezzmc.bake.nomenus")) {
                        finish(mc);
                        return;
                    }
                    probe.createWorld();
                }
                case 1 -> {
                    if (probe.ready()) {
                        phase = 2;
                    }
                }
                case 2 -> {
                    if (probe.tick()) {
                        finish(mc);
                    }
                }
                default -> {
                }
            }
        } catch (Throwable ex) {
            System.out.println("[ezzmc-bake] FAILED in phase " + phase);
            ex.printStackTrace(System.out);
            phase = 3;
            mc.stop();
        }
    }

    /** Пишет базу (v2-секции) + язык + меню (v3) и закрывает игру. */
    private void finish(Minecraft mc) throws IOException {
        phase = 3;
        Path out = Path.of(System.getProperty("ezzmc.bake.out"));
        Le tail = new Le();
        probe.writeLang(tail);
        probe.writeMenus(tail);
        byte[] t = tail.toBytes();
        byte[] data = java.util.Arrays.copyOf(base, base.length + t.length);
        System.arraycopy(t, 0, data, base.length, t.length);
        Path tmp = out.resolveSibling(out.getFileName() + ".tmp");
        Files.write(tmp, data);
        Files.move(tmp, out, java.nio.file.StandardCopyOption.REPLACE_EXISTING);
        System.out.println("[ezzmc-bake] OK " + out + " " + data.length + " bytes");
        mc.stop();
    }

    static final class Dumper {
        final Minecraft mc;
        final Le w = new Le();
        final Map<String, Integer> sprites = new HashMap<>();
        final List<TextureAtlasSprite> spriteList = new ArrayList<>();
        final Map<Key, Integer> models = new HashMap<>();
        final List<byte[]> modelList = new ArrayList<>();
        final Map<Key, Integer> shapes = new HashMap<>();
        final List<byte[]> shapeList = new ArrayList<>();
        final BlockColors colors;
        int errors;

        Dumper(Minecraft mc) {
            this.mc = mc;
            this.colors = mc.getBlockColors();
        }

        byte[] dump() throws IOException {
            // блоки в порядке реестра
            List<Block> blocks = new ArrayList<>();
            for (Block b : BuiltInRegistries.BLOCK) {
                blocks.add(b);
            }
            Map<Block, Integer> blockIdx = new HashMap<>();
            for (int i = 0; i < blocks.size(); i++) {
                blockIdx.put(blocks.get(i), i);
            }
            // состояния в порядке глобального id
            Le states = new Le();
            int nstates = Block.BLOCK_STATE_REGISTRY.size();
            states.u32(nstates);
            for (int id = 0; id < nstates; id++) {
                BlockState s = Block.BLOCK_STATE_REGISTRY.byId(id);
                if (s == null) {
                    states.u32(0xFFFFFFFF);
                    continue;
                }
                state(states, s, blockIdx.getOrDefault(s.getBlock(), -1));
            }

            w.bytes("EZBK".getBytes());
            w.u32(VERSION);
            atlas();
            w.u32(spriteList.size());
            for (TextureAtlasSprite sp : spriteList) {
                w.str(sp.contents().name().toString());
                w.u8(sp.atlasLocation().equals(TextureAtlas.LOCATION_BLOCKS) ? 0 : 1);
                w.u16(sp.getX());
                w.u16(sp.getY());
                w.u16(sp.contents().width());
                w.u16(sp.contents().height());
            }
            w.u32(blocks.size());
            for (Block b : blocks) {
                w.str(BuiltInRegistries.BLOCK.getKey(b).toString());
                var props = b.getStateDefinition().getProperties();
                w.u8(props.size());
                for (Property<?> p : props) {
                    w.str(p.getName());
                    var vals = p.getPossibleValues();
                    w.u16(vals.size());
                    for (Object v : vals) {
                        w.str(name(p, v));
                    }
                }
                w.u32(Block.getId(b.defaultBlockState()));
            }
            w.u32(modelList.size());
            for (byte[] m : modelList) {
                w.bytes(m);
            }
            w.u32(shapeList.size());
            for (byte[] m : shapeList) {
                w.bytes(m);
            }
            w.bytes(states.toBytes());
            items();
            System.out.println("[ezzmc-bake] blocks=" + blocks.size() + " states=" + nstates
                    + " models=" + modelList.size() + " shapes=" + shapeList.size()
                    + " sprites=" + spriteList.size() + " errors=" + errors);
            return w.toBytes();
        }

        /** Размер иконки предмета (px) и страницы атласа иконок. */
        static final int CELL = 32;
        static final int PAGE = 2048;

        /**
         * v2: иконки модовых предметов (TODO-508). Рисуются настоящим GuiGraphics#renderItem
         * в офскрин-цель (GUI-масштаб CELL/16), страница PAGE×PAGE, предмет i — клетка i на странице
         * i / (PAGE/CELL)^2, слева направо, сверху вниз.
         * Формат: u32 nitems { str name; u8 ok }; u32 cell; u32 page; u32 npages { u32 png_len; png }.
         */
        void items() throws IOException {
            List<Item> items = new ArrayList<>();
            for (Item it : BuiltInRegistries.ITEM) {
                if (!"minecraft".equals(BuiltInRegistries.ITEM.getKey(it).getNamespace())) {
                    items.add(it);
                }
            }
            int perRow = PAGE / CELL;
            int perPage = perRow * perRow;
            int npages = (items.size() + perPage - 1) / perPage;
            boolean[] ok = new boolean[items.size()];
            List<byte[]> pages = new ArrayList<>();
            TextureTarget target = new TextureTarget(PAGE, PAGE, true, Minecraft.ON_OSX);
            target.setClearColor(0, 0, 0, 0);
            RenderSystem.backupProjectionMatrix();
            PoseStack mv = RenderSystem.getModelViewStack();
            mv.pushPose();
            mv.setIdentity();
            mv.translate(0, 0, -11000);
            RenderSystem.applyModelViewMatrix();
            RenderSystem.setProjectionMatrix(new Matrix4f().setOrtho(0, PAGE, PAGE, 0, 1000, 21000),
                    VertexSorting.ORTHOGRAPHIC_Z);
            int itemErrors = 0;
            try {
                for (int p = 0; p < npages; p++) {
                    target.clear(Minecraft.ON_OSX);
                    target.bindWrite(true);
                    Lighting.setupFor3DItems();
                    GuiGraphics g = new GuiGraphics(mc, mc.renderBuffers().bufferSource());
                    int end = Math.min(items.size(), (p + 1) * perPage);
                    for (int i = p * perPage; i < end; i++) {
                        int c = i - p * perPage;
                        g.pose().pushPose();
                        g.pose().translate((c % perRow) * CELL, (c / perRow) * CELL, 0);
                        g.pose().scale(CELL / 16f, CELL / 16f, 1);
                        try {
                            g.renderItem(new ItemStack(items.get(i)), 0, 0);
                            g.flush();
                            ok[i] = true;
                        } catch (Throwable t) {
                            itemErrors++;
                            try {
                                mc.renderBuffers().bufferSource().endBatch();
                            } catch (Throwable ignored) {
                                // буфер уже сломан — следующий предмет начнёт заново
                            }
                        }
                        g.pose().popPose();
                    }
                    g.flush();
                    target.unbindWrite();
                    try (NativeImage img = new NativeImage(PAGE, PAGE, false)) {
                        RenderSystem.bindTexture(target.getColorTextureId());
                        img.downloadTexture(0, false);
                        img.flipY();
                        pages.add(img.asByteArray());
                    }
                }
            } finally {
                mv.popPose();
                RenderSystem.applyModelViewMatrix();
                RenderSystem.restoreProjectionMatrix();
                target.destroyBuffers();
                mc.getMainRenderTarget().bindWrite(true);
            }
            w.u32(items.size());
            for (int i = 0; i < items.size(); i++) {
                w.str(BuiltInRegistries.ITEM.getKey(items.get(i)).toString());
                w.u8(ok[i] ? 1 : 0);
            }
            w.u32(CELL);
            w.u32(PAGE);
            w.u32(pages.size());
            for (byte[] png : pages) {
                w.u32(png.length);
                w.bytes(png);
            }
            System.out.println("[ezzmc-bake] items=" + items.size() + " pages=" + pages.size()
                    + " item_errors=" + itemErrors);
        }

        @SuppressWarnings({"unchecked", "rawtypes"})
        static String name(Property p, Object v) {
            return p.getName((Comparable) v);
        }

        void atlas() throws IOException {
            TextureAtlas atlas = mc.getModelManager().getAtlas(TextureAtlas.LOCATION_BLOCKS);
            RenderSystem.bindTexture(atlas.getId());
            int aw = GL11.glGetTexLevelParameteri(GL11.GL_TEXTURE_2D, 0, GL11.GL_TEXTURE_WIDTH);
            int ah = GL11.glGetTexLevelParameteri(GL11.GL_TEXTURE_2D, 0, GL11.GL_TEXTURE_HEIGHT);
            try (NativeImage img = new NativeImage(aw, ah, false)) {
                img.downloadTexture(0, false);
                byte[] png = img.asByteArray();
                w.u32(aw);
                w.u32(ah);
                w.u32(png.length);
                w.bytes(png);
            }
        }

        void state(Le o, BlockState s, int block) {
            int flags = 0;
            int model = 0xFFFFFFFF;
            int maxTint = -1;
            try {
                BakedModel bm = mc.getBlockRenderer().getBlockModel(s);
                Le m = new Le();
                List<WeightedEntry.Wrapper<BakedModel>> variants = variants(bm);
                if (variants == null) {
                    m.u8(1);
                    m.u32(1);
                    maxTint = Math.max(maxTint, quads(m, bm, s));
                } else {
                    m.u8(variants.size());
                    for (WeightedEntry.Wrapper<BakedModel> v : variants) {
                        m.u32(v.getWeight().asInt());
                        maxTint = Math.max(maxTint, quads(m, v.getData(), s));
                    }
                }
                model = intern(models, modelList, m.toBytes());
                if (bm.useAmbientOcclusion()) {
                    flags |= 16;
                }
            } catch (Throwable t) {
                errors++;
                flags |= 128;
            }
            o.u32(block);
            o.u32(model);
            o.u32(shape(() -> s.getCollisionShape(EmptyBlockGetter.INSTANCE, BlockPos.ZERO)));
            o.u32(shape(() -> s.getShape(EmptyBlockGetter.INSTANCE, BlockPos.ZERO)));
            o.u32(shape(() -> s.getOcclusionShape(EmptyBlockGetter.INSTANCE, BlockPos.ZERO)));
            int light = 0;
            int lightBlock = 0;
            try {
                light = s.getLightEmission();
                lightBlock = s.getLightBlock(EmptyBlockGetter.INSTANCE, BlockPos.ZERO);
                if (s.canOcclude()) {
                    flags |= 1;
                }
                if (s.isSolidRender(EmptyBlockGetter.INSTANCE, BlockPos.ZERO)) {
                    flags |= 2;
                }
                if (s.isAir()) {
                    flags |= 4;
                }
                if (s.hasBlockEntity()) {
                    flags |= 8;
                }
                if (s.propagatesSkylightDown(EmptyBlockGetter.INSTANCE, BlockPos.ZERO)) {
                    flags |= 32;
                }
                if (!s.getFluidState().isEmpty()) {
                    flags |= 64;
                }
            } catch (Throwable t) {
                errors++;
                flags |= 128;
            }
            o.u8(light);
            o.u8(lightBlock);
            o.u8(flags);
            int nt = Math.min(maxTint + 1, 16);
            o.u8(nt);
            for (int i = 0; i < nt; i++) {
                // статус: 0 — нет обработчика (-1), 1 — цвет, 2 — нужен уровень (неизвестно как),
                // 3/4/5 — биомный цвет травы/листвы/воды (TODO-509: пробный уровень TintProbe)
                int st;
                int c = -1;
                try {
                    c = colors.getColor(s, null, null, i);
                    st = c == -1 ? 0 : 1;
                } catch (Throwable t) {
                    st = 2;
                }
                if (st == 2) {
                    st = TintProbe.classify(colors, s, i);
                }
                o.u8(st);
                o.u32(st == 1 || st == 0 ? c : -1);
            }
        }

        @SuppressWarnings("unchecked")
        static List<WeightedEntry.Wrapper<BakedModel>> variants(BakedModel bm) throws Exception {
            if (!(bm instanceof WeightedBakedModel)) {
                return null;
            }
            for (Field f : WeightedBakedModel.class.getDeclaredFields()) {
                if (f.getType() == List.class) {
                    f.setAccessible(true);
                    return (List<WeightedEntry.Wrapper<BakedModel>>) f.get(bm);
                }
            }
            return null;
        }

        static final Direction[] SIDES = {Direction.DOWN, Direction.UP, Direction.NORTH,
                Direction.SOUTH, Direction.WEST, Direction.EAST, null};

        /** Квады одного варианта; возвращает максимальный tintIndex. */
        int quads(Le m, BakedModel bm, BlockState s) {
            List<RenderType> types = new ArrayList<>();
            for (RenderType rt : bm.getRenderTypes(s, RandomSource.create(42), ModelData.EMPTY)) {
                types.add(rt);
            }
            Le q = new Le();
            int n = 0;
            int maxTint = -1;
            for (RenderType rt : types) {
                int layer = layer(rt);
                for (int si = 0; si < SIDES.length; si++) {
                    List<BakedQuad> list = bm.getQuads(s, SIDES[si], RandomSource.create(42),
                            ModelData.EMPTY, rt);
                    for (BakedQuad bq : list) {
                        quad(q, bq, si, layer);
                        maxTint = Math.max(maxTint, bq.getTintIndex());
                        n++;
                    }
                }
            }
            m.u32(n);
            m.bytes(q.toBytes());
            return maxTint;
        }

        static int layer(RenderType rt) {
            if (rt.equals(RenderType.solid())) return 0;
            if (rt.equals(RenderType.cutoutMipped())) return 1;
            if (rt.equals(RenderType.cutout())) return 2;
            if (rt.equals(RenderType.translucent())) return 3;
            if (rt.equals(RenderType.tripwire())) return 4;
            return 5;
        }

        void quad(Le q, BakedQuad bq, int cull, int layer) {
            TextureAtlasSprite sp = bq.getSprite();
            q.u8(cull);
            q.u8(bq.getDirection().ordinal());
            q.u8(bq.getTintIndex() & 0xFF);
            q.u8((bq.isShade() ? 1 : 0) | (bq.hasAmbientOcclusion() ? 2 : 0));
            q.u8(layer);
            q.u32(sprite(sp));
            int[] v = bq.getVertices();
            int stride = v.length / 4;
            float u0 = sp.getU0(), du = sp.getU1() - u0, v0 = sp.getV0(), dv = sp.getV1() - v0;
            for (int i = 0; i < 4; i++) {
                int b = i * stride;
                q.f32(Float.intBitsToFloat(v[b]));
                q.f32(Float.intBitsToFloat(v[b + 1]));
                q.f32(Float.intBitsToFloat(v[b + 2]));
                q.u32(v[b + 3]);
                q.f32(du == 0 ? 0 : (Float.intBitsToFloat(v[b + 4]) - u0) / du);
                q.f32(dv == 0 ? 0 : (Float.intBitsToFloat(v[b + 5]) - v0) / dv);
            }
        }

        int sprite(TextureAtlasSprite sp) {
            String key = sp.atlasLocation() + "|" + sp.contents().name();
            Integer i = sprites.get(key);
            if (i == null) {
                i = spriteList.size();
                sprites.put(key, i);
                spriteList.add(sp);
            }
            return i;
        }

        interface ShapeFn {
            VoxelShape get();
        }

        int shape(ShapeFn f) {
            Le o = new Le();
            try {
                List<AABB> boxes = f.get().toAabbs();
                o.u16(boxes.size());
                for (AABB b : boxes) {
                    o.f32((float) b.minX);
                    o.f32((float) b.minY);
                    o.f32((float) b.minZ);
                    o.f32((float) b.maxX);
                    o.f32((float) b.maxY);
                    o.f32((float) b.maxZ);
                }
            } catch (Throwable t) {
                errors++;
                return 0xFFFFFFFF;
            }
            return intern(shapes, shapeList, o.toBytes());
        }

        static int intern(Map<Key, Integer> map, List<byte[]> list, byte[] b) {
            Key k = new Key(b);
            Integer i = map.get(k);
            if (i == null) {
                i = list.size();
                map.put(k, i);
                list.add(b);
            }
            return i;
        }
    }

    record Key(byte[] b) {
        @Override
        public boolean equals(Object o) {
            return o instanceof Key k && java.util.Arrays.equals(b, k.b);
        }

        @Override
        public int hashCode() {
            return java.util.Arrays.hashCode(b);
        }
    }

    /** Little-endian буфер. */
    static final class Le {
        private byte[] buf = new byte[256];
        private int len;

        private void need(int n) {
            if (len + n > buf.length) {
                buf = java.util.Arrays.copyOf(buf, Math.max(buf.length * 2, len + n));
            }
        }

        void u8(int v) {
            need(1);
            buf[len++] = (byte) v;
        }

        void u16(int v) {
            need(2);
            buf[len++] = (byte) v;
            buf[len++] = (byte) (v >>> 8);
        }

        void u32(int v) {
            need(4);
            buf[len++] = (byte) v;
            buf[len++] = (byte) (v >>> 8);
            buf[len++] = (byte) (v >>> 16);
            buf[len++] = (byte) (v >>> 24);
        }

        void f32(float f) {
            u32(Float.floatToRawIntBits(f));
        }

        void bytes(byte[] b) {
            need(b.length);
            System.arraycopy(b, 0, buf, len, b.length);
            len += b.length;
        }

        void str(String s) {
            byte[] b = s.getBytes(java.nio.charset.StandardCharsets.UTF_8);
            u16(b.length);
            bytes(b);
        }

        byte[] toBytes() {
            return java.util.Arrays.copyOf(buf, len);
        }
    }
}
