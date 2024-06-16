use super::{symbol_table::Symbol, token::Location};

//Will still need to handle arrays
//I think I do need strings after all

//I think the AST also has to do with the symbol table

// #[derive(Debug,)]
// pub struct Literal;

// #[derive(Debug,)]
// pub struct FunctionCall;

// #[derive(Debug,)]
// pub struct FunctionDeclaration;

// #[derive(Debug,)]
// pub struct If;

// #[derive(Debug,)]
// pub struct While;

// #[derive(Debug,)]
// pub struct For;

// #[derive(Debug,)]
// pub struct Binary;

// #[derive(Debug,)]
// pub struct Declaration;

// #[derive(Debug,)]
// pub struct Return;

#[derive(Debug,)]
///Owned smart pointer on the heap.
pub struct P<T,> {
  ptr:Box<T,>,
}

impl<T,> P<T,> {
  pub fn new(val:T,) -> Self {
    P { ptr:Box::new(val,), }
  }
}
#[derive(Debug,)]
pub enum LiteralKind {
  Bool,
  Integer,
  Float,
  Str,
}

#[derive(Debug,)]
pub struct Literal {
  pub kind:LiteralKind,
  pub symbol:Symbol,
}

#[derive(Debug,)]
pub enum ExpressionKind {
  Literal(P<Literal,>,),
}

#[derive(Debug,)]
pub struct Expression {
  pub id:usize,
  pub kind:ExpressionKind,
  pub loc:Location,
}

#[derive(Debug,)]
pub enum Ty {
  Int,
  Float,
  Usize,
  String,
  Array(P<Ty,>,),
}

impl Ty {
  pub fn new(val:String,) -> Option<Self,> {
    match val.as_str() {
      "int" => Some(Ty::Int,),
      "float" => Some(Ty::Float,),
      "usize" => Some(Ty::Usize,),
      "string" => Some(Ty::String,),
      "int[]" => Some(Ty::Array(P::new(Ty::Int,),),),
      "float[]" => Some(Ty::Array(P::new(Ty::Float,),),),
      "usize[]" => Some(Ty::Array(P::new(Ty::Usize,),),),
      "string[]" => Some(Ty::Array(P::new(Ty::String,),),),
      _ => None,
    }
  }
}

#[derive(Debug,)]
pub struct Ident {
  pub name:String,
  pub loc:Location,
}

#[derive(Debug,)]
pub enum PatKind {
  Ident { mutable:bool, ident:Ident, },
}

#[derive(Debug,)]
pub struct Pat {
  pub id:usize,
  ///[`Location`] of the first element in the statement
  pub loc:Location,
  pub kind:PatKind,
}

#[derive(Debug,)]
pub enum LocalKind {
  ///Local declaration. Example: `let x;`
  Decl,
  ///Local declaration with an initializer. Example: `let x = y;`
  Init(P<Expression,>,),
}

#[derive(Debug,)]
pub struct Local {
  pub id:usize,
  ///[`Location`] of the first element in the statement
  pub loc:Location,
  ///Type of the local.
  pub ty:Option<P<Ty,>,>,
  pub pat:P<Pat,>,
  pub kind:LocalKind,
}

#[derive(Debug,)]
pub enum StatementKind {
  Let(P<Local,>,),
  Item(),
  Expression(Expression,),
}

#[derive(Debug,)]
pub struct Statement {
  pub id:usize,
  ///[`Location`] of the first element in the statement
  pub loc:Location,
  pub kind:StatementKind,
}

impl Statement {
  pub fn new(loc:Location, kind:StatementKind,) -> Self {
    Statement { id:0, loc, kind, }
  }
}

// #[derive(Debug,)]
// struct Node {
//   pub id:usize,
//   pub statement:Statement,
// }

#[derive(Debug,)]
pub struct AbstractSyntaxTree(Vec<Statement,>,);

impl AbstractSyntaxTree {
  pub fn new() -> Self {
    AbstractSyntaxTree(Vec::new(),)
  }

  pub fn push(&mut self, statement:Statement,) {
    // let node = Node { id:0, statement, };
    self.0.push(statement,);
  }
}

// #[derive(Clone, Encodable, Decodable, Debug)]
// pub struct Expr {
//     pub id: NodeId,
//     pub kind: ExprKind,
//     pub span: Span,
//     pub attrs: AttrVec,
//     pub tokens: Option<LazyAttrTokenStream>,
// }

// #[derive(Clone, Encodable, Decodable, Debug)]
// pub enum ExprKind {
//     /// An array (e.g, `[a, b, c, d]`).
//     Array(ThinVec<P<Expr>>),
//     /// Allow anonymous constants from an inline `const` block
//     ConstBlock(AnonConst),
//     /// A function call
//     ///
//     /// The first field resolves to the function itself,
//     /// and the second field is the list of arguments.
//     /// This also represents calling the constructor of
//     /// tuple-like ADTs such as tuple structs and enum variants.
//     Call(P<Expr>, ThinVec<P<Expr>>),
//     /// A method call (e.g., `x.foo::<Bar, Baz>(a, b, c)`).
//     MethodCall(Box<MethodCall>),
//     /// A tuple (e.g., `(a, b, c, d)`).
//     Tup(ThinVec<P<Expr>>),
//     /// A binary operation (e.g., `a + b`, `a * b`).
//     Binary(BinOp, P<Expr>, P<Expr>),
//     /// A unary operation (e.g., `!x`, `*x`).
//     Unary(UnOp, P<Expr>),
//     /// A literal (e.g., `1`, `"foo"`).
//     Lit(token::Lit),
//     /// A cast (e.g., `foo as f64`).
//     Cast(P<Expr>, P<Ty>),
//     /// A type ascription (e.g., `builtin # type_ascribe(42, usize)`).
//     ///
//     /// Usually not written directly in user code but
//     /// indirectly via the macro `type_ascribe!(...)`.
//     Type(P<Expr>, P<Ty>),
//     /// A `let pat = expr` expression that is only semantically allowed in
// the condition     /// of `if` / `while` expressions. (e.g., `if let 0 = x {
// .. }`).     ///
//     /// `Span` represents the whole `let pat = expr` statement.
//     Let(P<Pat>, P<Expr>, Span, Recovered),
//     /// An `if` block, with an optional `else` block.
//     ///
//     /// `if expr { block } else { expr }`
//     If(P<Expr>, P<Block>, Option<P<Expr>>),
//     /// A while loop, with an optional label.
//     ///
//     /// `'label: while expr { block }`
//     While(P<Expr>, P<Block>, Option<Label>),
//     /// A `for` loop, with an optional label.
//     ///
//     /// `'label: for await? pat in iter { block }`
//     ///
//     /// This is desugared to a combination of `loop` and `match` expressions.
//     ForLoop { pat: P<Pat>, iter: P<Expr>, body: P<Block>, label:
// Option<Label>, kind: ForLoopKind },     /// Conditionless loop (can be exited
// with `break`, `continue`, or `return`).     ///
//     /// `'label: loop { block }`
//     Loop(P<Block>, Option<Label>, Span),
//     /// A `match` block.
//     Match(P<Expr>, ThinVec<Arm>, MatchKind),
//     /// A closure (e.g., `move |a, b, c| a + b + c`).
//     Closure(Box<Closure>),
//     /// A block (`'label: { ... }`).
//     Block(P<Block>, Option<Label>),
//     /// An `async` block (`async move { ... }`),
//     /// or a `gen` block (`gen move { ... }`)
//     Gen(CaptureBy, P<Block>, GenBlockKind),
//     /// An await expression (`my_future.await`). Span is of await keyword.
//     Await(P<Expr>, Span),

//     /// A try block (`try { ... }`).
//     TryBlock(P<Block>),

//     /// An assignment (`a = foo()`).
//     /// The `Span` argument is the span of the `=` token.
//     Assign(P<Expr>, P<Expr>, Span),
//     /// An assignment with an operator.
//     ///
//     /// E.g., `a += 1`.
//     AssignOp(BinOp, P<Expr>, P<Expr>),
//     /// Access of a named (e.g., `obj.foo`) or unnamed (e.g., `obj.0`) struct
// field.     Field(P<Expr>, Ident),
//     /// An indexing operation (e.g., `foo[2]`).
//     /// The span represents the span of the `[2]`, including brackets.
//     Index(P<Expr>, P<Expr>, Span),
//     /// A range (e.g., `1..2`, `1..`, `..2`, `1..=2`, `..=2`; and `..` in
// destructuring assignment).     Range(Option<P<Expr>>, Option<P<Expr>>,
// RangeLimits),     /// An underscore, used in destructuring assignment to
// ignore a value.     Underscore,

//     /// Variable reference, possibly containing `::` and/or type
//     /// parameters (e.g., `foo::bar::<baz>`).
//     ///
//     /// Optionally "qualified" (e.g., `<Vec<T> as SomeTrait>::SomeType`).
//     Path(Option<P<QSelf>>, Path),

//     /// A referencing operation (`&a`, `&mut a`, `&raw const a` or `&raw mut
// a`).     AddrOf(BorrowKind, Mutability, P<Expr>),
//     /// A `break`, with an optional label to break, and an optional
// expression.     Break(Option<Label>, Option<P<Expr>>),
//     /// A `continue`, with an optional label.
//     Continue(Option<Label>),
//     /// A `return`, with an optional value to be returned.
//     Ret(Option<P<Expr>>),

//     /// Output of the `asm!()` macro.
//     InlineAsm(P<InlineAsm>),

//     /// An `offset_of` expression (e.g., `builtin # offset_of(Struct,
// field)`).     ///
//     /// Usually not written directly in user code but
//     /// indirectly via the macro `core::mem::offset_of!(...)`.
//     OffsetOf(P<Ty>, P<[Ident]>),

//     /// A macro invocation; pre-expansion.
//     MacCall(P<MacCall>),

//     /// A struct literal expression.
//     ///
//     /// E.g., `Foo {x: 1, y: 2}`, or `Foo {x: 1, .. rest}`.
//     Struct(P<StructExpr>),

//     /// An array literal constructed from one repeated element.
//     ///
//     /// E.g., `[1; 5]`. The expression is the element to be
//     /// repeated; the constant is the number of times to repeat it.
//     Repeat(P<Expr>, AnonConst),

//     /// No-op: used solely so we can pretty-print faithfully.
//     Paren(P<Expr>),

//     /// A try expression (`expr?`).
//     Try(P<Expr>),

//     /// A `yield`, with an optional value to be yielded.
//     Yield(Option<P<Expr>>),

//     /// A `do yeet` (aka `throw`/`fail`/`bail`/`raise`/whatever),
//     /// with an optional value to be returned.
//     Yeet(Option<P<Expr>>),

//     /// A tail call return, with the value to be returned.
//     ///
//     /// While `.0` must be a function call, we check this later, after
// parsing.     Become(P<Expr>),

//     /// Bytes included via `include_bytes!`
//     /// Added for optimization purposes to avoid the need to escape
//     /// large binary blobs - should always behave like [`ExprKind::Lit`]
//     /// with a `ByteStr` literal.
//     IncludedBytes(Lrc<[u8]>),

//     /// A `format_args!()` expression.
//     FormatArgs(P<FormatArgs>),

//     /// Placeholder for an expression that wasn't syntactically well formed
// in some way.     Err(ErrorGuaranteed),

//     /// Acts as a null expression. Lowering it will always emit a bug.
//     Dummy,
// }
