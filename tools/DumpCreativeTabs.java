import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;
import net.minecraft.world.item.*;
import net.minecraft.world.flag.FeatureFlags;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.data.registries.VanillaRegistries;
import net.minecraft.core.component.DataComponentPatch;
import net.minecraft.nbt.NbtOps;

public class Dump {
  public static void main(String[] a) throws Exception { try { run(); } catch (Throwable t) { StringBuilder sb=new StringBuilder(t.toString()); for (var e : t.getStackTrace()) sb.append("\n  at ").append(e); Throwable c=t.getCause(); while(c!=null){sb.append("\nCAUSE ").append(c); for (var e : c.getStackTrace()) {sb.append("\n  at ").append(e);} c=c.getCause();} java.nio.file.Files.writeString(java.nio.file.Path.of("trace.txt"), sb.toString()); } }
  static void run() throws Exception {
    SharedConstants.tryDetectVersion();
    Bootstrap.bootStrap();
    var rm = new net.minecraft.server.packs.resources.MultiPackResourceManager(net.minecraft.server.packs.PackType.SERVER_DATA, java.util.List.<net.minecraft.server.packs.PackResources>of(net.minecraft.server.packs.repository.ServerPacksSource.createVanillaPackSource().fullResources()));
    var layers = net.minecraft.server.RegistryLayer.createRegistryAccess();
    var pending = net.minecraft.tags.TagLoader.loadTagsForExistingRegistries(rm, layers.getLayer(net.minecraft.server.RegistryLayer.STATIC));
    var ctx = net.minecraft.tags.TagLoader.buildUpdatedLookups(layers.getAccessForLoading(net.minecraft.server.RegistryLayer.WORLD), pending);
    var world = net.minecraft.resources.RegistryDataLoader.load(rm, ctx, net.minecraft.resources.RegistryDataLoader.WORLD_REGISTRIES, Runnable::run).get();
    var all = java.util.stream.Stream.concat(ctx.stream(), world.listRegistries()).toList();
    net.minecraft.core.HolderLookup.Provider lookup = net.minecraft.core.HolderLookup.Provider.create(all.stream());
    var racc = layers.replaceFrom(net.minecraft.server.RegistryLayer.WORLD, world).compositeAccess();
    pending.forEach(net.minecraft.core.Registry.PendingTags::apply);
    net.minecraft.core.registries.BuiltInRegistries.DATA_COMPONENT_INITIALIZERS.build(lookup).forEach(net.minecraft.core.component.DataComponentInitializers.PendingComponents::apply);
    CreativeModeTabs.tryRebuildTabContents(FeatureFlags.DEFAULT_FLAGS, true, lookup);
    for (CreativeModeTab t : CreativeModeTabs.allTabs()) {
      String key = BuiltInRegistries.CREATIVE_MODE_TAB.getKey(t).getPath();
      System.out.println("#TAB " + key + " " + t.getDisplayItems().size());
      for (ItemStack s : t.getDisplayItems()) {
        String id = BuiltInRegistries.ITEM.getKey(s.getItem()).getPath();
        String p = "";
        if (!s.getComponentsPatch().isEmpty()) {
          var buf = new net.minecraft.network.RegistryFriendlyByteBuf(io.netty.buffer.Unpooled.buffer(), racc);
          ItemStack.OPTIONAL_UNTRUSTED_STREAM_CODEC.encode(buf, s);
          byte[] bytes = new byte[buf.readableBytes()]; buf.readBytes(bytes);
          // пропускаем count и item (varint), остаётся патч
          int i = 0; while ((bytes[i] & 0x80) != 0) i++; i++; while ((bytes[i] & 0x80) != 0) i++; i++;
          StringBuilder hx = new StringBuilder(" ");
          for (int k = i; k < bytes.length; k++) hx.append(String.format("%02x", bytes[k]));
          p = hx.toString();
        }
        System.out.println(id + p);
      }
    }
  }
}
