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
