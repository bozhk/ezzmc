public class Pot {
  public static void main(String[] a) throws Throwable {
    try { run(); } catch (Throwable t) { t.printStackTrace(new java.io.PrintStream(new java.io.FileOutputStream("trace2.txt"))); }
  }
  static void run() throws Throwable {
    net.minecraft.SharedConstants.tryDetectVersion();
    net.minecraft.server.Bootstrap.bootStrap();
    var out = new StringBuilder();
    for (var h : net.minecraft.core.registries.BuiltInRegistries.POTION.listElements().toList()) {
      var pc = new net.minecraft.world.item.alchemy.PotionContents(h);
      out.append(h.key().identifier().getPath()).append(' ').append(String.format("%06x", pc.getColor() & 0xFFFFFF)).append('\n');
    }
    java.nio.file.Files.writeString(java.nio.file.Path.of("potions.txt"), out.toString());
  }
}
