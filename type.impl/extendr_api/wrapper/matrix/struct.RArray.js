(function() {var type_impls = {
"extendr_api":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-RArray%3CT,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#26\">source</a><a href=\"#impl-Debug-for-RArray%3CT,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>, D: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, D&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#26\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/nightly/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","extendr_api::wrapper::matrix::RColumn","extendr_api::wrapper::matrix::RMatrix","extendr_api::wrapper::matrix::RMatrix3D"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deref-for-RArray%3CT,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#511-517\">source</a><a href=\"#impl-Deref-for-RArray%3CT,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, D&gt; <a class=\"trait\" href=\"extendr_api/trait.Deref.html\" title=\"trait extendr_api::Deref\">Deref</a> for <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, D&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Target\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Target\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"extendr_api/trait.Deref.html#associatedtype.Target\" class=\"associatedtype\">Target</a> = <a class=\"struct\" href=\"extendr_api/robj/struct.Robj.html\" title=\"struct extendr_api::robj::Robj\">Robj</a></h4></section></summary><div class='docblock'>The resulting type after dereferencing.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#514-516\">source</a><a href=\"#method.deref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"extendr_api/trait.Deref.html#tymethod.deref\" class=\"fn\">deref</a>(&amp;self) -&gt; &amp;Self::<a class=\"associatedtype\" href=\"extendr_api/trait.Deref.html#associatedtype.Target\" title=\"type extendr_api::Deref::Target\">Target</a></h4></section></summary><div class='docblock'>Dereferences the value.</div></details></div></details>","Deref","extendr_api::wrapper::matrix::RColumn","extendr_api::wrapper::matrix::RMatrix","extendr_api::wrapper::matrix::RMatrix3D"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DerefMut-for-RArray%3CT,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#519-523\">source</a><a href=\"#impl-DerefMut-for-RArray%3CT,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, D&gt; <a class=\"trait\" href=\"extendr_api/trait.DerefMut.html\" title=\"trait extendr_api::DerefMut\">DerefMut</a> for <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, D&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deref_mut\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#520-522\">source</a><a href=\"#method.deref_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"extendr_api/trait.DerefMut.html#tymethod.deref_mut\" class=\"fn\">deref_mut</a>(&amp;mut self) -&gt; &amp;mut Self::<a class=\"associatedtype\" href=\"extendr_api/trait.Deref.html#associatedtype.Target\" title=\"type extendr_api::Deref::Target\">Target</a></h4></section></summary><div class='docblock'>Mutably dereferences the value.</div></details></div></details>","DerefMut","extendr_api::wrapper::matrix::RColumn","extendr_api::wrapper::matrix::RMatrix","extendr_api::wrapper::matrix::RMatrix3D"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Index%3C%5Busize;+2%5D%3E-for-RArray%3CT,+%5Busize;+2%5D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#451-480\">source</a><a href=\"#impl-Index%3C%5Busize;+2%5D%3E-for-RArray%3CT,+%5Busize;+2%5D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html\" title=\"trait core::ops::index::Index\">Index</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2</a>]&gt; for <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2</a>]&gt;<div class=\"where\">where\n    <a class=\"struct\" href=\"extendr_api/robj/struct.Robj.html\" title=\"struct extendr_api::robj::Robj\">Robj</a>: for&lt;'a&gt; <a class=\"trait\" href=\"extendr_api/robj/trait.AsTypedSlice.html\" title=\"trait extendr_api::robj::AsTypedSlice\">AsTypedSlice</a>&lt;'a, T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.index\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#471-479\">source</a><a href=\"#method.index\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index\" class=\"fn\">index</a>(&amp;self, index: [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2</a>]) -&gt; &amp;Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output\" title=\"type core::ops::index::Index::Output\">Output</a></h4></section></summary><div class=\"docblock\"><p>Zero-based indexing in row, column order.</p>\n<p>Panics if out of bounds.</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>extendr_api::prelude::<span class=\"kw-2\">*</span>;\n<span class=\"macro\">test!</span> {\n   <span class=\"kw\">let </span>matrix = RArray::new_matrix(<span class=\"number\">3</span>, <span class=\"number\">2</span>, |r, c| [\n       [<span class=\"number\">1.</span>, <span class=\"number\">2.</span>, <span class=\"number\">3.</span>],\n       [<span class=\"number\">4.</span>, <span class=\"number\">5.</span>, <span class=\"number\">6.</span>]][c][r]);\n    <span class=\"macro\">assert_eq!</span>(matrix[[<span class=\"number\">0</span>, <span class=\"number\">0</span>]], <span class=\"number\">1.</span>);\n    <span class=\"macro\">assert_eq!</span>(matrix[[<span class=\"number\">1</span>, <span class=\"number\">0</span>]], <span class=\"number\">2.</span>);\n    <span class=\"macro\">assert_eq!</span>(matrix[[<span class=\"number\">2</span>, <span class=\"number\">1</span>]], <span class=\"number\">6.</span>);\n}</code></pre></div>\n</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Output\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Output\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output\" class=\"associatedtype\">Output</a> = T</h4></section></summary><div class='docblock'>The returned type after indexing.</div></details></div></details>","Index<[usize; 2]>","extendr_api::wrapper::matrix::RMatrix"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IndexMut%3C%5Busize;+2%5D%3E-for-RArray%3CT,+%5Busize;+2%5D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#482-509\">source</a><a href=\"#impl-IndexMut%3C%5Busize;+2%5D%3E-for-RArray%3CT,+%5Busize;+2%5D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html\" title=\"trait core::ops::index::IndexMut\">IndexMut</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2</a>]&gt; for <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2</a>]&gt;<div class=\"where\">where\n    <a class=\"struct\" href=\"extendr_api/robj/struct.Robj.html\" title=\"struct extendr_api::robj::Robj\">Robj</a>: for&lt;'a&gt; <a class=\"trait\" href=\"extendr_api/robj/trait.AsTypedSlice.html\" title=\"trait extendr_api::robj::AsTypedSlice\">AsTypedSlice</a>&lt;'a, T&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.index_mut\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#500-508\">source</a><a href=\"#method.index_mut\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut\" class=\"fn\">index_mut</a>(&amp;mut self, index: [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2</a>]) -&gt; &amp;mut Self::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output\" title=\"type core::ops::index::Index::Output\">Output</a></h4></section></summary><div class=\"docblock\"><p>Zero-based mutable indexing in row, column order.</p>\n<p>Panics if out of bounds.</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>extendr_api::prelude::<span class=\"kw-2\">*</span>;\n<span class=\"macro\">test!</span> {\n    <span class=\"kw\">let </span><span class=\"kw-2\">mut </span>matrix = RMatrix::new_matrix(<span class=\"number\">3</span>, <span class=\"number\">2</span>, |<span class=\"kw\">_</span>, <span class=\"kw\">_</span>| <span class=\"number\">0.</span>);\n    matrix[[<span class=\"number\">0</span>, <span class=\"number\">0</span>]] = <span class=\"number\">1.</span>;\n    matrix[[<span class=\"number\">1</span>, <span class=\"number\">0</span>]] = <span class=\"number\">2.</span>;\n    matrix[[<span class=\"number\">2</span>, <span class=\"number\">0</span>]] = <span class=\"number\">3.</span>;\n    matrix[[<span class=\"number\">0</span>, <span class=\"number\">1</span>]] = <span class=\"number\">4.</span>;\n    <span class=\"macro\">assert_eq!</span>(matrix.as_real_slice().unwrap(), <span class=\"kw-2\">&amp;</span>[<span class=\"number\">1.</span>, <span class=\"number\">2.</span>, <span class=\"number\">3.</span>, <span class=\"number\">4.</span>, <span class=\"number\">0.</span>, <span class=\"number\">0.</span>]);\n}</code></pre></div>\n</div></details></div></details>","IndexMut<[usize; 2]>","extendr_api::wrapper::matrix::RMatrix"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Offset%3C%5Busize;+1%5D%3E-for-RArray%3CT,+%5Busize;+1%5D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#134-142\">source</a><a href=\"#impl-Offset%3C%5Busize;+1%5D%3E-for-RArray%3CT,+%5Busize;+1%5D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"extendr_api/wrapper/matrix/trait.Offset.html\" title=\"trait extendr_api::wrapper::matrix::Offset\">Offset</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">1</a>]&gt; for <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">1</a>]&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.offset\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#136-141\">source</a><a href=\"#method.offset\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"extendr_api/wrapper/matrix/trait.Offset.html#tymethod.offset\" class=\"fn\">offset</a>(&amp;self, index: [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">1</a>]) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>Get the offset into the array for a given index.</p>\n</div></details></div></details>","Offset<[usize; 1]>","extendr_api::wrapper::matrix::RColumn"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Offset%3C%5Busize;+2%5D%3E-for-RArray%3CT,+%5Busize;+2%5D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#144-155\">source</a><a href=\"#impl-Offset%3C%5Busize;+2%5D%3E-for-RArray%3CT,+%5Busize;+2%5D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"extendr_api/wrapper/matrix/trait.Offset.html\" title=\"trait extendr_api::wrapper::matrix::Offset\">Offset</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2</a>]&gt; for <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2</a>]&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.offset\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#146-154\">source</a><a href=\"#method.offset\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"extendr_api/wrapper/matrix/trait.Offset.html#tymethod.offset\" class=\"fn\">offset</a>(&amp;self, index: [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">2</a>]) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>Get the offset into the array for a given index.</p>\n</div></details></div></details>","Offset<[usize; 2]>","extendr_api::wrapper::matrix::RMatrix"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Offset%3C%5Busize;+3%5D%3E-for-RArray%3CT,+%5Busize;+3%5D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#157-171\">source</a><a href=\"#impl-Offset%3C%5Busize;+3%5D%3E-for-RArray%3CT,+%5Busize;+3%5D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"extendr_api/wrapper/matrix/trait.Offset.html\" title=\"trait extendr_api::wrapper::matrix::Offset\">Offset</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">3</a>]&gt; for <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">3</a>]&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.offset\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#159-170\">source</a><a href=\"#method.offset\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"extendr_api/wrapper/matrix/trait.Offset.html#tymethod.offset\" class=\"fn\">offset</a>(&amp;self, index: [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.array.html\">3</a>]) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a></h4></section></summary><div class=\"docblock\"><p>Get the offset into the array for a given index.</p>\n</div></details></div></details>","Offset<[usize; 3]>","extendr_api::wrapper::matrix::RMatrix3D"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-RArray%3CT,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#26\">source</a><a href=\"#impl-PartialEq-for-RArray%3CT,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>, D: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, D&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#26\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, D&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","extendr_api::wrapper::matrix::RColumn","extendr_api::wrapper::matrix::RMatrix","extendr_api::wrapper::matrix::RMatrix3D"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RArray%3CT,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#173-199\">source</a><a href=\"#impl-RArray%3CT,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, D&gt; <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, D&gt;<div class=\"where\">where\n    <a class=\"struct\" href=\"extendr_api/robj/struct.Robj.html\" title=\"struct extendr_api::robj::Robj\">Robj</a>: for&lt;'a&gt; <a class=\"trait\" href=\"extendr_api/robj/trait.AsTypedSlice.html\" title=\"trait extendr_api::robj::AsTypedSlice\">AsTypedSlice</a>&lt;'a, T&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.from_parts\" class=\"method\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#177-183\">source</a><h4 class=\"code-header\">pub fn <a href=\"extendr_api/wrapper/matrix/struct.RArray.html#tymethod.from_parts\" class=\"fn\">from_parts</a>(robj: <a class=\"struct\" href=\"extendr_api/robj/struct.Robj.html\" title=\"struct extendr_api::robj::Robj\">Robj</a>, dim: D) -&gt; Self</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.data\" class=\"method\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#186-188\">source</a><h4 class=\"code-header\">pub fn <a href=\"extendr_api/wrapper/matrix/struct.RArray.html#tymethod.data\" class=\"fn\">data</a>(&amp;self) -&gt; &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[T]</a></h4></section></summary><div class=\"docblock\"><p>Returns a flat representation of the array in col-major.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.data_mut\" class=\"method\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#191-193\">source</a><h4 class=\"code-header\">pub fn <a href=\"extendr_api/wrapper/matrix/struct.RArray.html#tymethod.data_mut\" class=\"fn\">data_mut</a>(&amp;mut self) -&gt; &amp;mut <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[T]</a></h4></section></summary><div class=\"docblock\"><p>Returns a flat, mutable representation of the array in col-major.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.dim\" class=\"method\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#196-198\">source</a><h4 class=\"code-header\">pub fn <a href=\"extendr_api/wrapper/matrix/struct.RArray.html#tymethod.dim\" class=\"fn\">dim</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;D</a></h4></section></summary><div class=\"docblock\"><p>Get the dimensions for this array.</p>\n</div></details></div></details>",0,"extendr_api::wrapper::matrix::RColumn","extendr_api::wrapper::matrix::RMatrix","extendr_api::wrapper::matrix::RMatrix3D"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RArray%3CT,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#37-70\">source</a><a href=\"#impl-RArray%3CT,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, D&gt; <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, D&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.get_dimnames\" class=\"method\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#38-40\">source</a><h4 class=\"code-header\">pub fn <a href=\"extendr_api/wrapper/matrix/struct.RArray.html#tymethod.get_dimnames\" class=\"fn\">get_dimnames</a>(&amp;self) -&gt; <a class=\"struct\" href=\"extendr_api/wrapper/list/struct.List.html\" title=\"struct extendr_api::wrapper::list::List\">List</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_names\" class=\"method\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#46-49\">source</a><h4 class=\"code-header\">pub fn <a href=\"extendr_api/wrapper/matrix/struct.RArray.html#tymethod.set_names\" class=\"fn\">set_names</a>(&amp;mut self, names: <a class=\"struct\" href=\"extendr_api/wrapper/strings/struct.Strings.html\" title=\"struct extendr_api::wrapper::strings::Strings\">Strings</a>)</h4></section></summary><div class=\"docblock\"><p>Set the names of the elements of an array.</p>\n<p>Equivalent to <code>names&lt;-</code> in R</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_dimnames\" class=\"method\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#58-60\">source</a><h4 class=\"code-header\">pub fn <a href=\"extendr_api/wrapper/matrix/struct.RArray.html#tymethod.set_dimnames\" class=\"fn\">set_dimnames</a>(&amp;mut self, dimnames: <a class=\"struct\" href=\"extendr_api/wrapper/list/struct.List.html\" title=\"struct extendr_api::wrapper::list::List\">List</a>)</h4></section></summary><div class=\"docblock\"><p>Set the dimension names of an array.</p>\n<p>For <a href=\"extendr_api/wrapper/matrix/type.RMatrix.html\" title=\"type extendr_api::wrapper::matrix::RMatrix\"><code>RMatrix</code></a> a list of length 2 is required, as that would entail\ncolumn-names and row-names. If you only wish to set one, but not the other,\nthen the unset element must be R <code>NULL</code></p>\n<p>Equivalent to <code>dimnames&lt;-</code> in R</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_dim\" class=\"method\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#65-69\">source</a><h4 class=\"code-header\">pub fn <a href=\"extendr_api/wrapper/matrix/struct.RArray.html#tymethod.set_dim\" class=\"fn\">set_dim</a>(&amp;mut self, dim: <a class=\"struct\" href=\"extendr_api/robj/struct.Robj.html\" title=\"struct extendr_api::robj::Robj\">Robj</a>)</h4></section></summary><div class=\"docblock\"><p>Set the dimensions of an array.</p>\n<p>Equivalent to <code>dim&lt;-</code></p>\n</div></details></div></details>",0,"extendr_api::wrapper::matrix::RColumn","extendr_api::wrapper::matrix::RMatrix","extendr_api::wrapper::matrix::RMatrix3D"],["<section id=\"impl-StructuralPartialEq-for-RArray%3CT,+D%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/extendr_api/wrapper/matrix.rs.html#26\">source</a><a href=\"#impl-StructuralPartialEq-for-RArray%3CT,+D%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"extendr_api/wrapper/matrix/struct.RArray.html\" title=\"struct extendr_api::wrapper::matrix::RArray\">RArray</a>&lt;T, D&gt;</h3></section>","StructuralPartialEq","extendr_api::wrapper::matrix::RColumn","extendr_api::wrapper::matrix::RMatrix","extendr_api::wrapper::matrix::RMatrix3D"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()