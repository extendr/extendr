//! Statics representing R Symbols
use crate::SEXP;

extern "C" {
    #[doc = "\"base\""]
    pub static mut R_BaseSymbol: SEXP;
    #[doc = "\"\" as a STRSXP"]
    pub static mut R_BlankScalarString: SEXP;
    #[doc = "\"\" as a CHARSXP"]
    pub static mut R_BlankString: SEXP;
    #[doc = "\"{\""]
    pub static mut R_BraceSymbol: SEXP;
    #[doc = "\"\\[\\[\""]
    pub static mut R_Bracket2Symbol: SEXP;
    #[doc = "\"\\[\""]
    pub static mut R_BracketSymbol: SEXP;
    #[doc = "\"class\""]
    pub static mut R_ClassSymbol: SEXP;
    #[doc = "\".Device\""]
    pub static mut R_DeviceSymbol: SEXP;
    #[doc = "\"dimnames\""]
    pub static mut R_DimNamesSymbol: SEXP;
    #[doc = "\"dim\""]
    pub static mut R_DimSymbol: SEXP;
    #[doc = "\"$\""]
    pub static mut R_DollarSymbol: SEXP;
    #[doc = "\"...\""]
    pub static mut R_DotsSymbol: SEXP;
    #[doc = "\"::\""]
    pub static mut R_DoubleColonSymbol: SEXP;
    #[doc = "\"drop\""]
    pub static mut R_DropSymbol: SEXP;
    #[doc = "\"eval\""]
    pub static mut R_EvalSymbol: SEXP;
    #[doc = "\"function\""]
    pub static mut R_FunctionSymbol: SEXP;
    #[doc = "\".Last.value\""]
    pub static mut R_LastvalueSymbol: SEXP;
    #[doc = "\"levels\""]
    pub static mut R_LevelsSymbol: SEXP;
    #[doc = "\"mode\""]
    pub static mut R_ModeSymbol: SEXP;
    #[doc = "\"na.rm\""]
    pub static mut R_NaRmSymbol: SEXP;
    #[doc = "\"name\""]
    pub static mut R_NameSymbol: SEXP;
    #[doc = "\"names\""]
    pub static mut R_NamesSymbol: SEXP;
    #[doc = "\".__NAMESPACE__.\""]
    pub static mut R_NamespaceEnvSymbol: SEXP;
    #[doc = "\"package\""]
    pub static mut R_PackageSymbol: SEXP;
    #[doc = "\"previous\""]
    pub static mut R_PreviousSymbol: SEXP;
    #[doc = "\"quote\""]
    pub static mut R_QuoteSymbol: SEXP;
    #[doc = "\"row.names\""]
    pub static mut R_RowNamesSymbol: SEXP;
    #[doc = "\".Random.seed\""]
    pub static mut R_SeedsSymbol: SEXP;
    #[doc = "\"sort.list\""]
    pub static mut R_SortListSymbol: SEXP;
    #[doc = "\"source\""]
    pub static mut R_SourceSymbol: SEXP;
    #[doc = "\"spec\""]
    pub static mut R_SpecSymbol: SEXP;
    #[doc = "\":::\""]
    pub static mut R_TripleColonSymbol: SEXP;
    #[doc = "\"tsp\""]
    pub static mut R_TspSymbol: SEXP;
    #[doc = "\".defined\""]
    pub static mut R_dot_defined: SEXP;
    #[doc = "\".Generic\""]
    pub static mut R_dot_Generic: SEXP;
    #[doc = "\".Method\""]
    pub static mut R_dot_Method: SEXP;
    #[doc = "\".packageName\""]
    pub static mut R_dot_packageName: SEXP;

    #[doc = "\".target\""]
    pub static mut R_dot_target: SEXP;
}
