package fr.supersurviveur.rustcraftmod.rustapi.rustblock;

import com.mojang.serialization.MapCodec;
import fr.supersurviveur.rustcraftmod.rustapi.RustAPI;
import net.minecraft.block.Block;
import net.minecraft.block.BlockEntityProvider;
import net.minecraft.block.BlockState;
import net.minecraft.block.BlockWithEntity;
import net.minecraft.block.entity.BlockEntity;
import net.minecraft.entity.Entity;
import net.minecraft.entity.player.PlayerEntity;
import net.minecraft.item.ItemStack;
import net.minecraft.state.property.BooleanProperty;
import net.minecraft.text.Text;
import net.minecraft.util.ActionResult;
import net.minecraft.util.Hand;
import net.minecraft.util.hit.BlockHitResult;
import net.minecraft.util.math.BlockPos;
import net.minecraft.world.World;

public class RustBlock extends Block implements BlockEntityProvider {
    long rust_object;
    RustAPI rustAPI;

    public RustBlock(Settings settings, RustAPI rustAPI, long block) {
        super(settings);
        this.rustAPI = rustAPI;
        this.rust_object = block;
    }


    @Override
    public BlockEntity createBlockEntity(BlockPos pos, BlockState state) {
        return new RustBlockEntity(pos, state);
    }

    @Override
    protected ActionResult onUseWithItem(ItemStack stack, BlockState state, World world, BlockPos pos, PlayerEntity player, Hand hand, BlockHitResult hit) {
        if (world.isClient) return ActionResult.SUCCESS;

        if (!(world.getBlockEntity(pos) instanceof RustBlockEntity blockEntity)) {
            return ActionResult.PASS_TO_DEFAULT_BLOCK_ACTION;
        }

        if (!player.getStackInHand(hand).isEmpty()) {
            // Check what is the first open slot and put an item from the player's hand there
            if (blockEntity.getStack(0).isEmpty()) {
                // Put the stack the player is holding into the inventory
                blockEntity.setStack(0, player.getStackInHand(hand).copy());
                // Remove the stack from the player's hand
                player.getStackInHand(hand).setCount(0);
            } else if (blockEntity.getStack(1).isEmpty()) {
                blockEntity.setStack(1, player.getStackInHand(hand).copy());
                player.getStackInHand(hand).setCount(0);
            } else {
                // If the inventory is full we'll notify the player
                player.sendMessage(Text.literal("The inventory is full! The first slot holds ")
                        .append(blockEntity.getStack(0).getName())
                        .append(" and the second slot holds ")
                        .append(blockEntity.getStack(1).getName()), true);
            }
        } else {
            // If the player is not holding anything we'll get give him the items in the block entity one by one

            // Find the first slot that has an item and give it to the player
            if (!blockEntity.getStack(1).isEmpty()) {
                // Give the player the stack in the inventory
                player.getInventory().offerOrDrop(blockEntity.getStack(1));
                // Remove the stack from the inventory
                blockEntity.removeStack(1);
            } else if (!blockEntity.getStack(0).isEmpty()) {
                player.getInventory().offerOrDrop(blockEntity.getStack(0));
                blockEntity.removeStack(0);
            } else {
                return ActionResult.PASS_TO_DEFAULT_BLOCK_ACTION;
            }
        }
        return ActionResult.SUCCESS;
    }

}
