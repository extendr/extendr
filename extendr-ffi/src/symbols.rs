//! Statics representing R Symbols
use crate::SEXP;

extern "C" {
    #[doc = "\"base\""]
    pub static R_BaseSymbol: SEXP;
    #[doc = "\"\" as a STRSXP"]
    pub static R_BlankScalarString: SEXP;
    #[doc = "\"\" as a CHARSXP"]
    pub static R_BlankString: SEXP;
    #[doc = "\"{\""]
    pub static R_BraceSymbol: SEXP;
    #[doc = "\"\\[\\[\""]
    pub static R_Bracket2Symbol: SEXP;
    #[doc = "\"\\[\""]
    pub static R_BracketSymbol: SEXP;
    #[doc = "\"class\""]
    pub static R_ClassSymbol: SEXP;
    #[doc = "\".Device\""]
    pub static R_DeviceSymbol: SEXP;
    #[doc = "\"dimnames\""]
    pub static R_DimNamesSymbol: SEXP;
    #[doc = "\"dim\""]
    pub static R_DimSymbol: SEXP;
    #[doc = "\"$\""]
    pub static R_DollarSymbol: SEXP;
    #[doc = "\"...\""]
    pub static R_DotsSymbol: SEXP;
    #[doc = "\"::\""]
    pub static R_DoubleColonSymbol: SEXP;
    #[doc = "\"drop\""]
    pub static R_DropSymbol: SEXP;
    #[doc = "\"eval\""]
    pub static R_EvalSymbol: SEXP;
    #[doc = "\"function\""]
    pub static R_FunctionSymbol: SEXP;
    #[doc = "\".Last.value\""]
    pub static R_LastvalueSymbol: SEXP;
    #[doc = "\"levels\""]
    pub static R_LevelsSymbol: SEXP;
    #[doc = "\"mode\""]
    pub static R_ModeSymbol: SEXP;
    #[doc = "\"na.rm\""]
    pub static R_NaRmSymbol: SEXP;
    #[doc = "\"name\""]
    pub static R_NameSymbol: SEXP;
    #[doc = "\"names\""]
    pub static R_NamesSymbol: SEXP;
    #[doc = "\".__NAMESPACE__.\""]
    pub static R_NamespaceEnvSymbol: SEXP;
    #[doc = "\"package\""]
    pub static R_PackageSymbol: SEXP;
    #[doc = "\"previous\""]
    pub static R_PreviousSymbol: SEXP;
    #[doc = "\"quote\""]
    pub static R_QuoteSymbol: SEXP;
    #[doc = "\"row.names\""]
    pub static R_RowNamesSymbol: SEXP;
    #[doc = "\".Random.seed\""]
    pub static R_SeedsSymbol: SEXP;
    #[doc = "\"sort.list\""]
    pub static R_SortListSymbol: SEXP;
    #[doc = "\"source\""]
    pub static R_SourceSymbol: SEXP;
    #[doc = "\"spec\""]
    pub static R_SpecSymbol: SEXP;
    #[doc = "\":::\""]
    pub static R_TripleColonSymbol: SEXP;
    #[doc = "\"tsp\""]
    pub static R_TspSymbol: SEXP;
    #[doc = "\".defined\""]
    pub static R_dot_defined: SEXP;
    #[doc = "\".Generic\""]
    pub static R_dot_Generic: SEXP;
    #[doc = "\".Method\""]
    pub static R_dot_Method: SEXP;
    #[doc = "\".packageName\""]
    pub static R_dot_packageName: SEXP;

    #[doc = "\".target\""]
    pub static R_dot_target: SEXP;
}
