---
title: Raytracing - Rust Proseminar
author: Sami Shalayel, Daniel Freiermuth, Carl Schwan
---

# Projektstruktur

## Interceptable Trait

Wir können alles rendern, was den ,,Interceptable''-Trait implementiert :

Dieser gibt an, ob, wo und wie das Objekt von einem Lichtstrahl getroffen wird.
    
. . .

Beispiele für ,,Interceptable'':

+ Kugeln
+ Ebenen
+ Dreiecke
+ Beschleunigungsstrukturen

---

## Shader Trait


was der shader so zum rendern braucht (dh auch Distanz zum Objekt, Winkel zur Oberfläche,  ...)
</div>

Jeder Shader liefert eine Farbe für einen Lichtstrahl-Objekt-Schnitt.

. . .
    
Wir haben folgende Shader implementiert :

+ Monochrome shader
+ Additive shader
+ Multiplicative shader
+ Chess shader
+ Diffuse shader
+ Mirror shader
+ Specular shader

---

## Phong Shader

<div class="notes">
    Monochrome shader: "einfacher Shader"\\
    Additive shader: "S1+S2"\\
    Multiplicative shader: "S1*S2" bzw "Farbe * Shader"\\
    Chess shader: 2 alternierende subshader\\
    Diffuse shader: Diffuses Licht\\
    Mirror shader: Spiegelt das Licht zurück, wie ein Spiegel\\
    Specular shader: Spiegelungen\\
</div>

Die std::ops::Add und std::ops::Mul Traits für Box<Shader> erleichtern das Bauen des Phong-Shader :

```rust
pub fn get_phong(color: Vector3<f64>) -> Box<Shader> {
    let diffuse_shader = DiffuseShader::new(color);
    let specular_shader = SpecularShader::new(10.0);
    let ambient_shader = AmbientShader::new(color);
    return 0.5 * diffuse_shader + specular_shader + 0.8 * ambient_shader;
}
```

---

### Beispiel

\begin{columns}[t]
    \column{.5\textwidth}
    \centering
    \includegraphics[height=3cm]{example-true-ambiant} \\
    \includegraphics[height=3cm]{example-mirror}
    \column{.5\textwidth}
    \centering
    \includegraphics[height=3cm]{example-mirror2}
\end{columns}

--- 

## Camera Trait

```rust
pub trait Camera {
    fn render(&self, world: &World) -> DynamicImage;
}
```

+ Equilinear Camera
+ Equirectangular Camera

---

## Wavefront Parser

+ Wavefront Obj parser
+ Wavefront Mtl parser

Benutzt die wavefront\_obj crate

--- 

## Beispiel

\includegraphics[height=3cm]{example-duck}

# Rust in unsere Projekt 

## Operator overload

```rust
impl Add for Box<Shader> {
    type Output = Box<Shader>;
    fn add(self, other: Box<Shader>) -> Box<Shader> {
        Box::new(AdditiveShader {
            shader1: self,
            shader2: other,
        })
    }
}
```

---

## Nalgebra und std::f64
    
TODO

---

## Error Handling 

TODO

# Störend in Rust
+ Mehr Arbeit durch Borrow-Checker/Lifetimes
+ Rayon

# Lessons learned
+ Reference in Struct -> Lifetime
+ Serde
+ ...

# Frage?

TODO Video
