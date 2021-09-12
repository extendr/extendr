pub struct Color(i32);

impl Color {
    pub fn to_i32(&self) -> i32 {
        self.0
    }
}

pub fn web(hex: u32) -> Color {
    let red = (hex >> 16) & 0xff;
    let green = (hex >> 8) & 0xff;
    let blue = hex & 0xff;
    Color(red as i32 | (green as i32) << 8 | (blue as i32) << 16 | 0xff << 24)
}

pub fn web4(hex: u32) -> Color {
    let red = ((hex >> 8) & 0xf) * 0xff / 0x0f;
    let green = ((hex >> 4) & 0xf) * 0xff / 0x0f;
    let blue = (hex & 0xf) * 0xff / 0x0f;
    Color(red as i32 | (green as i32) << 8 | (blue as i32) << 16 | 0xff << 24)
}

pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
    Color(red as i32 | (green as i32) << 8 | (blue as i32) << 16 | 0xff << 24)
}

pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
    Color(red as i32 | (green as i32) << 8 | (blue as i32) << 16 | (alpha as i32) << 24)
}

/// https://www.w3.org/TR/2018/REC-css-color-3-20180619/
pub fn aliceblue() -> Color {
    web(0xF0F8FF)
}
pub fn antiquewhite() -> Color {
    web(0xFAEBD7)
}
pub fn aqua() -> Color {
    web(0x00FFFF)
}
pub fn aquamarine() -> Color {
    web(0x7FFFD4)
}
pub fn azure() -> Color {
    web(0xF0FFFF)
}
pub fn beige() -> Color {
    web(0xF5F5DC)
}
pub fn bisque() -> Color {
    web(0xFFE4C4)
}
pub fn black() -> Color {
    web(0x000000)
}
pub fn blanchedalmond() -> Color {
    web(0xFFEBCD)
}
pub fn blue() -> Color {
    web(0x0000FF)
}
pub fn blueviolet() -> Color {
    web(0x8A2BE2)
}
pub fn brown() -> Color {
    web(0xA52A2A)
}
pub fn burlywood() -> Color {
    web(0xDEB887)
}
pub fn cadetblue() -> Color {
    web(0x5F9EA0)
}
pub fn chartreuse() -> Color {
    web(0x7FFF00)
}
pub fn chocolate() -> Color {
    web(0xD2691E)
}
pub fn coral() -> Color {
    web(0xFF7F50)
}
pub fn cornflowerblue() -> Color {
    web(0x6495ED)
}
pub fn cornsilk() -> Color {
    web(0xFFF8DC)
}
pub fn crimson() -> Color {
    web(0xDC143C)
}
pub fn cyan() -> Color {
    web(0x00FFFF)
}
pub fn darkblue() -> Color {
    web(0x00008B)
}
pub fn darkcyan() -> Color {
    web(0x008B8B)
}
pub fn darkgoldenrod() -> Color {
    web(0xB8860B)
}
pub fn darkgray() -> Color {
    web(0xA9A9A9)
}
pub fn darkgreen() -> Color {
    web(0x006400)
}
pub fn darkgrey() -> Color {
    web(0xA9A9A9)
}
pub fn darkkhaki() -> Color {
    web(0xBDB76B)
}
pub fn darkmagenta() -> Color {
    web(0x8B008B)
}
pub fn darkolivegreen() -> Color {
    web(0x556B2F)
}
pub fn darkorange() -> Color {
    web(0xFF8C00)
}
pub fn darkorchid() -> Color {
    web(0x9932CC)
}
pub fn darkred() -> Color {
    web(0x8B0000)
}
pub fn darksalmon() -> Color {
    web(0xE9967A)
}
pub fn darkseagreen() -> Color {
    web(0x8FBC8F)
}
pub fn darkslateblue() -> Color {
    web(0x483D8B)
}
pub fn darkslategray() -> Color {
    web(0x2F4F4F)
}
pub fn darkslategrey() -> Color {
    web(0x2F4F4F)
}
pub fn darkturquoise() -> Color {
    web(0x00CED1)
}
pub fn darkviolet() -> Color {
    web(0x9400D3)
}
pub fn deeppink() -> Color {
    web(0xFF1493)
}
pub fn deepskyblue() -> Color {
    web(0x00BFFF)
}
pub fn dimgray() -> Color {
    web(0x696969)
}
pub fn dimgrey() -> Color {
    web(0x696969)
}
pub fn dodgerblue() -> Color {
    web(0x1E90FF)
}
pub fn firebrick() -> Color {
    web(0xB22222)
}
pub fn floralwhite() -> Color {
    web(0xFFFAF0)
}
pub fn forestgreen() -> Color {
    web(0x228B22)
}
pub fn fuchsia() -> Color {
    web(0xFF00FF)
}
pub fn gainsboro() -> Color {
    web(0xDCDCDC)
}
pub fn ghostwhite() -> Color {
    web(0xF8F8FF)
}
pub fn gold() -> Color {
    web(0xFFD700)
}
pub fn goldenrod() -> Color {
    web(0xDAA520)
}
pub fn gray() -> Color {
    web(0x808080)
}
pub fn green() -> Color {
    web(0x008000)
}
pub fn greenyellow() -> Color {
    web(0xADFF2F)
}
pub fn grey() -> Color {
    web(0x808080)
}
pub fn honeydew() -> Color {
    web(0xF0FFF0)
}
pub fn hotpink() -> Color {
    web(0xFF69B4)
}
pub fn indianred() -> Color {
    web(0xCD5C5C)
}
pub fn indigo() -> Color {
    web(0x4B0082)
}
pub fn ivory() -> Color {
    web(0xFFFFF0)
}
pub fn khaki() -> Color {
    web(0xF0E68C)
}
pub fn lavender() -> Color {
    web(0xE6E6FA)
}
pub fn lavenderblush() -> Color {
    web(0xFFF0F5)
}
pub fn lawngreen() -> Color {
    web(0x7CFC00)
}
pub fn lemonchiffon() -> Color {
    web(0xFFFACD)
}
pub fn lightblue() -> Color {
    web(0xADD8E6)
}
pub fn lightcoral() -> Color {
    web(0xF08080)
}
pub fn lightcyan() -> Color {
    web(0xE0FFFF)
}
pub fn lightgoldenrodyellow() -> Color {
    web(0xFAFAD2)
}
pub fn lightgray() -> Color {
    web(0xD3D3D3)
}
pub fn lightgreen() -> Color {
    web(0x90EE90)
}
pub fn lightgrey() -> Color {
    web(0xD3D3D3)
}
pub fn lightpink() -> Color {
    web(0xFFB6C1)
}
pub fn lightsalmon() -> Color {
    web(0xFFA07A)
}
pub fn lightseagreen() -> Color {
    web(0x20B2AA)
}
pub fn lightskyblue() -> Color {
    web(0x87CEFA)
}
pub fn lightslategray() -> Color {
    web(0x778899)
}
pub fn lightslategrey() -> Color {
    web(0x778899)
}
pub fn lightsteelblue() -> Color {
    web(0xB0C4DE)
}
pub fn lightyellow() -> Color {
    web(0xFFFFE0)
}
pub fn lime() -> Color {
    web(0x00FF00)
}
pub fn limegreen() -> Color {
    web(0x32CD32)
}
pub fn linen() -> Color {
    web(0xFAF0E6)
}
pub fn magenta() -> Color {
    web(0xFF00FF)
}
pub fn maroon() -> Color {
    web(0x800000)
}
pub fn mediumaquamarine() -> Color {
    web(0x66CDAA)
}
pub fn mediumblue() -> Color {
    web(0x0000CD)
}
pub fn mediumorchid() -> Color {
    web(0xBA55D3)
}
pub fn mediumpurple() -> Color {
    web(0x9370DB)
}
pub fn mediumseagreen() -> Color {
    web(0x3CB371)
}
pub fn mediumslateblue() -> Color {
    web(0x7B68EE)
}
pub fn mediumspringgreen() -> Color {
    web(0x00FA9A)
}
pub fn mediumturquoise() -> Color {
    web(0x48D1CC)
}
pub fn mediumvioletred() -> Color {
    web(0xC71585)
}
pub fn midnightblue() -> Color {
    web(0x191970)
}
pub fn mintcream() -> Color {
    web(0xF5FFFA)
}
pub fn mistyrose() -> Color {
    web(0xFFE4E1)
}
pub fn moccasin() -> Color {
    web(0xFFE4B5)
}
pub fn navajowhite() -> Color {
    web(0xFFDEAD)
}
pub fn navy() -> Color {
    web(0x000080)
}
pub fn oldlace() -> Color {
    web(0xFDF5E6)
}
pub fn olive() -> Color {
    web(0x808000)
}
pub fn olivedrab() -> Color {
    web(0x6B8E23)
}
pub fn orange() -> Color {
    web(0xFFA500)
}
pub fn orangered() -> Color {
    web(0xFF4500)
}
pub fn orchid() -> Color {
    web(0xDA70D6)
}
pub fn palegoldenrod() -> Color {
    web(0xEEE8AA)
}
pub fn palegreen() -> Color {
    web(0x98FB98)
}
pub fn paleturquoise() -> Color {
    web(0xAFEEEE)
}
pub fn palevioletred() -> Color {
    web(0xDB7093)
}
pub fn papayawhip() -> Color {
    web(0xFFEFD5)
}
pub fn peachpuff() -> Color {
    web(0xFFDAB9)
}
pub fn peru() -> Color {
    web(0xCD853F)
}
pub fn pink() -> Color {
    web(0xFFC0CB)
}
pub fn plum() -> Color {
    web(0xDDA0DD)
}
pub fn powderblue() -> Color {
    web(0xB0E0E6)
}
pub fn purple() -> Color {
    web(0x800080)
}
pub fn red() -> Color {
    web(0xFF0000)
}
pub fn rosybrown() -> Color {
    web(0xBC8F8F)
}
pub fn royalblue() -> Color {
    web(0x4169E1)
}
pub fn saddlebrown() -> Color {
    web(0x8B4513)
}
pub fn salmon() -> Color {
    web(0xFA8072)
}
pub fn sandybrown() -> Color {
    web(0xF4A460)
}
pub fn seagreen() -> Color {
    web(0x2E8B57)
}
pub fn seashell() -> Color {
    web(0xFFF5EE)
}
pub fn sienna() -> Color {
    web(0xA0522D)
}
pub fn silver() -> Color {
    web(0xC0C0C0)
}
pub fn skyblue() -> Color {
    web(0x87CEEB)
}
pub fn slateblue() -> Color {
    web(0x6A5ACD)
}
pub fn slategray() -> Color {
    web(0x708090)
}
pub fn slategrey() -> Color {
    web(0x708090)
}
pub fn snow() -> Color {
    web(0xFFFAFA)
}
pub fn springgreen() -> Color {
    web(0x00FF7F)
}
pub fn steelblue() -> Color {
    web(0x4682B4)
}
pub fn tan() -> Color {
    web(0xD2B48C)
}
pub fn teal() -> Color {
    web(0x008080)
}
pub fn thistle() -> Color {
    web(0xD8BFD8)
}
pub fn tomato() -> Color {
    web(0xFF6347)
}
pub fn turquoise() -> Color {
    web(0x40E0D0)
}
pub fn violet() -> Color {
    web(0xEE82EE)
}
pub fn wheat() -> Color {
    web(0xF5DEB3)
}
pub fn white() -> Color {
    web(0xFFFFFF)
}
pub fn whitesmoke() -> Color {
    web(0xF5F5F5)
}
pub fn yellow() -> Color {
    web(0xFFFF00)
}
pub fn yellowgreen() -> Color {
    web(0x9ACD32)
}
