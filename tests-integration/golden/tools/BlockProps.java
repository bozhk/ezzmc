// Снимает свойства КАЖДОГО состояния блока с настоящей ванили (1.21.x). Запуск: cargo xtask capture-golden <ver> BlockProps
// Формат: {"shapes":[[[minx,miny,minz,maxx,maxy,maxz],..],..], "states":[[destroy,lightEmit,lightBlock,flags,collShape,outlineShape,resist,friction,speed,jump],..]}
// flags: бит0 canOcclude, бит1 propagatesSkylightDown, бит2 requiresCorrectTool, бит3 isAir, бит4 isRandomlyTicking, бит5 blocksMotion, бит6 isSolid.
import java.util.*;
import net.minecraft.SharedConstants;
import net.minecraft.core.BlockPos;
import net.minecraft.server.Bootstrap;
import net.minecraft.world.level.EmptyBlockGetter;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.shapes.VoxelShape;

public class BlockProps {
    static final Map<String, Integer> shapeIds = new LinkedHashMap<>();

    static int shape(VoxelShape s) {
        StringBuilder sb = new StringBuilder("[");
        List<AABB> l = s.toAabbs();
        for (int i = 0; i < l.size(); i++) {
            AABB b = l.get(i);
            sb.append(String.format("%s[%s,%s,%s,%s,%s,%s]", i > 0 ? "," : "", b.minX, b.minY, b.minZ, b.maxX, b.maxY, b.maxZ));
        }
        return shapeIds.computeIfAbsent(sb.append("]").toString(), k -> shapeIds.size());
    }

    public static void main(String[] a) {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
        int n = 0;
        for (BlockState ignored : Block.BLOCK_STATE_REGISTRY) n++;
        StringBuilder st = new StringBuilder();
        for (int id = 0; id < n; id++) {
            BlockState s = Block.BLOCK_STATE_REGISTRY.byId(id);
            Block b = s.getBlock();
            int flags = (s.canOcclude() ? 1 : 0) | (s.propagatesSkylightDown() ? 2 : 0) | (s.requiresCorrectToolForDrops() ? 4 : 0)
                | (s.isAir() ? 8 : 0) | (s.isRandomlyTicking() ? 16 : 0) | ((b != Blocks.COBWEB && b != Blocks.BAMBOO_SAPLING && s.isSolid()) ? 32 : 0) | (s.isSolid() ? 64 : 0);
            st.append(String.format("%s[%s,%d,%d,%d,%d,%d,%s,%s,%s,%s]", id > 0 ? ",\n" : "",
                s.getDestroySpeed(EmptyBlockGetter.INSTANCE, BlockPos.ZERO), s.getLightEmission(), s.getLightDampening(), flags,
                shape(s.getCollisionShape(EmptyBlockGetter.INSTANCE, BlockPos.ZERO)),
                shape(s.getShape(EmptyBlockGetter.INSTANCE, BlockPos.ZERO)),
                b.getExplosionResistance(), b.getFriction(), b.getSpeedFactor(), b.getJumpFactor()));
        }
        StringBuilder sh = new StringBuilder();
        for (String k : shapeIds.keySet()) sh.append(sh.length() > 0 ? ",\n" : "").append(k);
        System.out.print("{\"shapes\":[\n" + sh + "\n],\n\"states\":[\n" + st + "\n]}\n");
    }
}
