// Снимает с настоящей ванили (Mth.sin/cos, таблица) значения как биты float. Запуск: cargo xtask capture-golden <ver> MthSin
import net.minecraft.util.Mth;

public class MthSin {
    public static void main(String[] a) {
        double[] xs = {0.0, 0.5, 1.0, -1.0, 3.14159265358979, 6.2831853, 100.0, -1234.5678, 0.0001, 1.5707963};
        StringBuilder sb = new StringBuilder("{\n  \"fn\": \"Mth.sin/cos\",\n  \"cases\": [\n");
        for (int i = 0; i < xs.length; i++) {
            sb.append(String.format("    {\"x\": %s, \"sin\": %d, \"cos\": %d}%s\n", Double.toString(xs[i]),
                Float.floatToRawIntBits(Mth.sin(xs[i])), Float.floatToRawIntBits(Mth.cos(xs[i])), i + 1 < xs.length ? "," : ""));
        }
        System.out.print(sb.append("  ]\n}\n"));
    }
}
