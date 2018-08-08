# Typed Lambda Calculus
ラムダ項の型を推論する

```
$ cargo run
> \x.x
\x:\tau_{0}.x : \tau_{0}->\tau_{0}

bussproofs format:
\begin{prooftree}
  \AxiomC{ $ \{ x : \tau_{0} \} \vdash x : \tau_{0} $ }
  \UnaryInfC{ $  \vdash \lambda x .x : \tau_{0} \to \tau_{0} $ }
\end{prooftree}
```

![Imgur](https://i.imgur.com/eIuku3I.png)

```
> \f.\x.f (f x)
\f:\tau_{5}->\tau_{5}.\x:\tau_{5}.f (f x) : (\tau_{5}->\tau_{5})->\tau_{5}->\tau_{5}

bussproofs format:
\begin{prooftree}
  \AxiomC{ $ \{ f : \tau_{5} \to \tau_{5},x : \tau_{5} \} \vdash f : \tau_{5} \to \tau_{5} $ }
  \AxiomC{ $ \{ f : \tau_{5} \to \tau_{5},x : \tau_{5} \} \vdash f : \tau_{5} \to \tau_{5} $ }
  \AxiomC{ $ \{ f : \tau_{5} \to \tau_{5},x : \tau_{5} \} \vdash x : \tau_{5} $ }
  \BinaryInfC{ $ \{ f : \tau_{5} \to \tau_{5},x : \tau_{5} \} \vdash f \, x : \tau_{5} $ }
  \BinaryInfC{ $ \{ f : \tau_{5} \to \tau_{5},x : \tau_{5} \} \vdash f \, \left(f \, x\right) : \tau_{5} $ }
  \UnaryInfC{ $ \{ f : \tau_{5} \to \tau_{5} \} \vdash \lambda x .f \, \left(f \, x\right) : \tau_{5} \to \tau_{5} $ }
  \UnaryInfC{ $  \vdash \lambda f .\lambda x .f \, \left(f \, x\right) : \left(\tau_{5} \to \tau_{5}\right) \to \tau_{5} \to \tau_{5} $ }
\end{prooftree}
```

![Imgur](https://i.imgur.com/eKhc3uN.png)
