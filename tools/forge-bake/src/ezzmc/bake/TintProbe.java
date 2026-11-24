package ezzmc.bake;

import net.minecraft.client.color.block.BlockColors;
import net.minecraft.client.renderer.BiomeColors;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.world.level.BlockAndTintGetter;
import net.minecraft.world.level.ColorResolver;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.lighting.LevelLightEngine;
import net.minecraft.world.level.material.FluidState;
import net.minecraft.world.level.material.Fluids;

/**
 * TODO-509: какой биомный цвет берёт обработчик тинта мода. Пробный уровень отвечает на
 * {@code getBlockTint(pos, resolver)} меткой по ванильному резолверу (BiomeColors: трава, листва,
 * вода); если обработчик вернул ровно метку — тинт = этот биомный цвет.
 */
final class TintProbe implements BlockAndTintGetter {
    static final int GRASS = 0x0E2A01, FOLIAGE = 0x0E2A02, WATER = 0x0E2A03, OTHER = 0x0E2A04;

    private final BlockState state;

    private TintProbe(BlockState state) {
        this.state = state;
    }

    /** 3 — трава, 4 — листва, 5 — вода, 2 — не распознано. */
    static int classify(BlockColors colors, BlockState s, int index) {
        try {
            int c = colors.getColor(s, new TintProbe(s), BlockPos.ZERO, index) & 0xFFFFFF;
            if (c == GRASS) {
                return 3;
            }
            if (c == FOLIAGE) {
                return 4;
            }
            if (c == WATER) {
                return 5;
            }
        } catch (Throwable ignored) {
            // обработчик лезет в блок-сущность/свет — неизвестно
        }
        return 2;
    }

    @Override
    public int getBlockTint(BlockPos pos, ColorResolver r) {
        if (r == BiomeColors.GRASS_COLOR_RESOLVER) {
            return GRASS;
        }
        if (r == BiomeColors.FOLIAGE_COLOR_RESOLVER) {
            return FOLIAGE;
        }
        if (r == BiomeColors.WATER_COLOR_RESOLVER) {
            return WATER;
        }
        return OTHER;
    }

    @Override
    public float getShade(Direction d, boolean shade) {
        return 1f;
    }

    @Override
    public LevelLightEngine getLightEngine() {
        return null;
    }

    @Override
    public BlockEntity getBlockEntity(BlockPos pos) {
        return null;
    }

    @Override
    public BlockState getBlockState(BlockPos pos) {
        return pos.equals(BlockPos.ZERO) ? state : Blocks.AIR.defaultBlockState();
    }

    @Override
    public FluidState getFluidState(BlockPos pos) {
        return Fluids.EMPTY.defaultFluidState();
    }

    @Override
    public int getHeight() {
        return 384;
    }

    @Override
    public int getMinBuildHeight() {
        return -64;
    }
}
