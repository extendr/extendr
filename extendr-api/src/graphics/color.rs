pub struct Color(i32);

impl Color {
    /// Convert to an R color.
    pub fn to_i32(&self) -> i32 {
        self.0
    }

    /// Generate a color from a CSS-like hex number.
    /// eg. `Color::hex(0xF0F8FF)`
    pub fn hex(hex: u32) -> Color {
        let red = (hex >> 16) & 0xff;
        let green = (hex >> 8) & 0xff;
        let blue = hex & 0xff;
        Color(red as i32 | (green as i32) << 8 | (blue as i32) << 16 | 0xff << 24)
    }

    /// Generate a color from a 3 digit CSS-like hex number.
    /// eg. `Color::hex(0xF0F)`
    pub fn hex3(hex: u32) -> Color {
        let red = ((hex >> 8) & 0xf) * 0xff / 0x0f;
        let green = ((hex >> 4) & 0xf) * 0xff / 0x0f;
        let blue = (hex & 0xf) * 0xff / 0x0f;
        Color(red as i32 | (green as i32) << 8 | (blue as i32) << 16 | 0xff << 24)
    }

    /// Generate a color from rgb components (0-255).
    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color(red as i32 | (green as i32) << 8 | (blue as i32) << 16 | 0xff << 24)
    }

    /// Generate a color from rgba components (0-255).
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color(red as i32 | (green as i32) << 8 | (blue as i32) << 16 | (alpha as i32) << 24)
    }
}

pub mod predefined {
    use super::Color;

    /// <https://www.w3.org/TR/2018/REC-css-color-3-20180619/>
    pub fn aliceblue() -> Color {
        Color::hex(0xF0F8FF)
    }
    pub fn antiquewhite() -> Color {
        Color::hex(0xFAEBD7)
    }
    pub fn aqua() -> Color {
        Color::hex(0x00FFFF)
    }
    pub fn aquamarine() -> Color {
        Color::hex(0x7FFFD4)
    }
    pub fn azure() -> Color {
        Color::hex(0xF0FFFF)
    }
    pub fn beige() -> Color {
        Color::hex(0xF5F5DC)
    }
    pub fn bisque() -> Color {
        Color::hex(0xFFE4C4)
    }
    pub fn black() -> Color {
        Color::hex(0x000000)
    }
    pub fn blanchedalmond() -> Color {
        Color::hex(0xFFEBCD)
    }
    pub fn blue() -> Color {
        Color::hex(0x0000FF)
    }
    pub fn blueviolet() -> Color {
        Color::hex(0x8A2BE2)
    }
    pub fn brown() -> Color {
        Color::hex(0xA52A2A)
    }
    pub fn burlywood() -> Color {
        Color::hex(0xDEB887)
    }
    pub fn cadetblue() -> Color {
        Color::hex(0x5F9EA0)
    }
    pub fn chartreuse() -> Color {
        Color::hex(0x7FFF00)
    }
    pub fn chocolate() -> Color {
        Color::hex(0xD2691E)
    }
    pub fn coral() -> Color {
        Color::hex(0xFF7F50)
    }
    pub fn cornflowerblue() -> Color {
        Color::hex(0x6495ED)
    }
    pub fn cornsilk() -> Color {
        Color::hex(0xFFF8DC)
    }
    pub fn crimson() -> Color {
        Color::hex(0xDC143C)
    }
    pub fn cyan() -> Color {
        Color::hex(0x00FFFF)
    }
    pub fn darkblue() -> Color {
        Color::hex(0x00008B)
    }
    pub fn darkcyan() -> Color {
        Color::hex(0x008B8B)
    }
    pub fn darkgoldenrod() -> Color {
        Color::hex(0xB8860B)
    }
    pub fn darkgray() -> Color {
        Color::hex(0xA9A9A9)
    }
    pub fn darkgreen() -> Color {
        Color::hex(0x006400)
    }
    pub fn darkgrey() -> Color {
        Color::hex(0xA9A9A9)
    }
    pub fn darkkhaki() -> Color {
        Color::hex(0xBDB76B)
    }
    pub fn darkmagenta() -> Color {
        Color::hex(0x8B008B)
    }
    pub fn darkolivegreen() -> Color {
        Color::hex(0x556B2F)
    }
    pub fn darkorange() -> Color {
        Color::hex(0xFF8C00)
    }
    pub fn darkorchid() -> Color {
        Color::hex(0x9932CC)
    }
    pub fn darkred() -> Color {
        Color::hex(0x8B0000)
    }
    pub fn darksalmon() -> Color {
        Color::hex(0xE9967A)
    }
    pub fn darkseagreen() -> Color {
        Color::hex(0x8FBC8F)
    }
    pub fn darkslateblue() -> Color {
        Color::hex(0x483D8B)
    }
    pub fn darkslategray() -> Color {
        Color::hex(0x2F4F4F)
    }
    pub fn darkslategrey() -> Color {
        Color::hex(0x2F4F4F)
    }
    pub fn darkturquoise() -> Color {
        Color::hex(0x00CED1)
    }
    pub fn darkviolet() -> Color {
        Color::hex(0x9400D3)
    }
    pub fn deeppink() -> Color {
        Color::hex(0xFF1493)
    }
    pub fn deepskyblue() -> Color {
        Color::hex(0x00BFFF)
    }
    pub fn dimgray() -> Color {
        Color::hex(0x696969)
    }
    pub fn dimgrey() -> Color {
        Color::hex(0x696969)
    }
    pub fn dodgerblue() -> Color {
        Color::hex(0x1E90FF)
    }
    pub fn firebrick() -> Color {
        Color::hex(0xB22222)
    }
    pub fn floralwhite() -> Color {
        Color::hex(0xFFFAF0)
    }
    pub fn forestgreen() -> Color {
        Color::hex(0x228B22)
    }
    pub fn fuchsia() -> Color {
        Color::hex(0xFF00FF)
    }
    pub fn gainsboro() -> Color {
        Color::hex(0xDCDCDC)
    }
    pub fn ghostwhite() -> Color {
        Color::hex(0xF8F8FF)
    }
    pub fn gold() -> Color {
        Color::hex(0xFFD700)
    }
    pub fn goldenrod() -> Color {
        Color::hex(0xDAA520)
    }
    pub fn gray() -> Color {
        Color::hex(0x808080)
    }
    pub fn green() -> Color {
        Color::hex(0x008000)
    }
    pub fn greenyellow() -> Color {
        Color::hex(0xADFF2F)
    }
    pub fn grey() -> Color {
        Color::hex(0x808080)
    }
    pub fn honeydew() -> Color {
        Color::hex(0xF0FFF0)
    }
    pub fn hotpink() -> Color {
        Color::hex(0xFF69B4)
    }
    pub fn indianred() -> Color {
        Color::hex(0xCD5C5C)
    }
    pub fn indigo() -> Color {
        Color::hex(0x4B0082)
    }
    pub fn ivory() -> Color {
        Color::hex(0xFFFFF0)
    }
    pub fn khaki() -> Color {
        Color::hex(0xF0E68C)
    }
    pub fn lavender() -> Color {
        Color::hex(0xE6E6FA)
    }
    pub fn lavenderblush() -> Color {
        Color::hex(0xFFF0F5)
    }
    pub fn lawngreen() -> Color {
        Color::hex(0x7CFC00)
    }
    pub fn lemonchiffon() -> Color {
        Color::hex(0xFFFACD)
    }
    pub fn lightblue() -> Color {
        Color::hex(0xADD8E6)
    }
    pub fn lightcoral() -> Color {
        Color::hex(0xF08080)
    }
    pub fn lightcyan() -> Color {
        Color::hex(0xE0FFFF)
    }
    pub fn lightgoldenrodyellow() -> Color {
        Color::hex(0xFAFAD2)
    }
    pub fn lightgray() -> Color {
        Color::hex(0xD3D3D3)
    }
    pub fn lightgreen() -> Color {
        Color::hex(0x90EE90)
    }
    pub fn lightgrey() -> Color {
        Color::hex(0xD3D3D3)
    }
    pub fn lightpink() -> Color {
        Color::hex(0xFFB6C1)
    }
    pub fn lightsalmon() -> Color {
        Color::hex(0xFFA07A)
    }
    pub fn lightseagreen() -> Color {
        Color::hex(0x20B2AA)
    }
    pub fn lightskyblue() -> Color {
        Color::hex(0x87CEFA)
    }
    pub fn lightslategray() -> Color {
        Color::hex(0x778899)
    }
    pub fn lightslategrey() -> Color {
        Color::hex(0x778899)
    }
    pub fn lightsteelblue() -> Color {
        Color::hex(0xB0C4DE)
    }
    pub fn lightyellow() -> Color {
        Color::hex(0xFFFFE0)
    }
    pub fn lime() -> Color {
        Color::hex(0x00FF00)
    }
    pub fn limegreen() -> Color {
        Color::hex(0x32CD32)
    }
    pub fn linen() -> Color {
        Color::hex(0xFAF0E6)
    }
    pub fn magenta() -> Color {
        Color::hex(0xFF00FF)
    }
    pub fn maroon() -> Color {
        Color::hex(0x800000)
    }
    pub fn mediumaquamarine() -> Color {
        Color::hex(0x66CDAA)
    }
    pub fn mediumblue() -> Color {
        Color::hex(0x0000CD)
    }
    pub fn mediumorchid() -> Color {
        Color::hex(0xBA55D3)
    }
    pub fn mediumpurple() -> Color {
        Color::hex(0x9370DB)
    }
    pub fn mediumseagreen() -> Color {
        Color::hex(0x3CB371)
    }
    pub fn mediumslateblue() -> Color {
        Color::hex(0x7B68EE)
    }
    pub fn mediumspringgreen() -> Color {
        Color::hex(0x00FA9A)
    }
    pub fn mediumturquoise() -> Color {
        Color::hex(0x48D1CC)
    }
    pub fn mediumvioletred() -> Color {
        Color::hex(0xC71585)
    }
    pub fn midnightblue() -> Color {
        Color::hex(0x191970)
    }
    pub fn mintcream() -> Color {
        Color::hex(0xF5FFFA)
    }
    pub fn mistyrose() -> Color {
        Color::hex(0xFFE4E1)
    }
    pub fn moccasin() -> Color {
        Color::hex(0xFFE4B5)
    }
    pub fn navajowhite() -> Color {
        Color::hex(0xFFDEAD)
    }
    pub fn navy() -> Color {
        Color::hex(0x000080)
    }
    pub fn oldlace() -> Color {
        Color::hex(0xFDF5E6)
    }
    pub fn olive() -> Color {
        Color::hex(0x808000)
    }
    pub fn olivedrab() -> Color {
        Color::hex(0x6B8E23)
    }
    pub fn orange() -> Color {
        Color::hex(0xFFA500)
    }
    pub fn orangered() -> Color {
        Color::hex(0xFF4500)
    }
    pub fn orchid() -> Color {
        Color::hex(0xDA70D6)
    }
    pub fn palegoldenrod() -> Color {
        Color::hex(0xEEE8AA)
    }
    pub fn palegreen() -> Color {
        Color::hex(0x98FB98)
    }
    pub fn paleturquoise() -> Color {
        Color::hex(0xAFEEEE)
    }
    pub fn palevioletred() -> Color {
        Color::hex(0xDB7093)
    }
    pub fn papayawhip() -> Color {
        Color::hex(0xFFEFD5)
    }
    pub fn peachpuff() -> Color {
        Color::hex(0xFFDAB9)
    }
    pub fn peru() -> Color {
        Color::hex(0xCD853F)
    }
    pub fn pink() -> Color {
        Color::hex(0xFFC0CB)
    }
    pub fn plum() -> Color {
        Color::hex(0xDDA0DD)
    }
    pub fn powderblue() -> Color {
        Color::hex(0xB0E0E6)
    }
    pub fn purple() -> Color {
        Color::hex(0x800080)
    }
    pub fn red() -> Color {
        Color::hex(0xFF0000)
    }
    pub fn rosybrown() -> Color {
        Color::hex(0xBC8F8F)
    }
    pub fn royalblue() -> Color {
        Color::hex(0x4169E1)
    }
    pub fn saddlebrown() -> Color {
        Color::hex(0x8B4513)
    }
    pub fn salmon() -> Color {
        Color::hex(0xFA8072)
    }
    pub fn sandybrown() -> Color {
        Color::hex(0xF4A460)
    }
    pub fn seagreen() -> Color {
        Color::hex(0x2E8B57)
    }
    pub fn seashell() -> Color {
        Color::hex(0xFFF5EE)
    }
    pub fn sienna() -> Color {
        Color::hex(0xA0522D)
    }
    pub fn silver() -> Color {
        Color::hex(0xC0C0C0)
    }
    pub fn skyblue() -> Color {
        Color::hex(0x87CEEB)
    }
    pub fn slateblue() -> Color {
        Color::hex(0x6A5ACD)
    }
    pub fn slategray() -> Color {
        Color::hex(0x708090)
    }
    pub fn slategrey() -> Color {
        Color::hex(0x708090)
    }
    pub fn snow() -> Color {
        Color::hex(0xFFFAFA)
    }
    pub fn springgreen() -> Color {
        Color::hex(0x00FF7F)
    }
    pub fn steelblue() -> Color {
        Color::hex(0x4682B4)
    }
    pub fn tan() -> Color {
        Color::hex(0xD2B48C)
    }
    pub fn teal() -> Color {
        Color::hex(0x008080)
    }
    pub fn thistle() -> Color {
        Color::hex(0xD8BFD8)
    }
    pub fn tomato() -> Color {
        Color::hex(0xFF6347)
    }
    pub fn turquoise() -> Color {
        Color::hex(0x40E0D0)
    }
    pub fn violet() -> Color {
        Color::hex(0xEE82EE)
    }
    pub fn wheat() -> Color {
        Color::hex(0xF5DEB3)
    }
    pub fn white() -> Color {
        Color::hex(0xFFFFFF)
    }
    pub fn whitesmoke() -> Color {
        Color::hex(0xF5F5F5)
    }
    pub fn yellow() -> Color {
        Color::hex(0xFFFF00)
    }
    pub fn yellowgreen() -> Color {
        Color::hex(0x9ACD32)
    }
}
