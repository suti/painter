# painter

`painter` is a simply 2D graphics library. api interface like [canvas api](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)

[online demo](https://codesandbox.io/s/painter-wasm-demo-6hgmrr?file=/index.html)

# feature

## CanvasRenderingContext2D
- arc ✅
- arcTo ❌
- beginPath ✅
- bezierCurveTo ✅
- canvas ✅
- clearRect ✅
- clip ✅
  - path(parameter) ✅
- closePath ✅
- createConicGradient ❌ 
- createImageData ❌
- createLinearGradient ❌
- createPattern ❌
- createRadialGradient ❌
- direction ❌
- drawFocusIfNeeded ❌
- drawImage ❌
  - ImageBitmap as source image ❌
  - SVGImageElement as source image ❌
- ellipse ❌
- fill ✅
  - path parameter ✅
- fillRect ✅
- fillStyle ✅basic
- fillText ✅
- filter ❌
- font ✅basic
- fontKerning ❌
- fontStretchExperimental ❌
- fontVariantCapsExperimental ❌
- getContextAttributes
- getImageData ❌
- getLineDash ❌
- getTransform ✅
- globalAlpha ❌
- globalCompositeOperation ❌
- imageSmoothingEnabled ❌
- imageSmoothingQuality ❌
- isContextLostExperimental ❌
- isPointInPath ❌
  - path parameter ❌
- isPointInStroke ❌
  - path parameter ❌
- letterSpacingExperimental ❌
- lineCap ✅
- lineDashOffset ❌
- lineJoin ✅
- lineTo ✅
- lineWidth ✅
- measureText ❌
- miterLimit ✅
- moveTo ✅
- putImageData
- quadraticCurveTo ✅
- rect ✅
- resetExperimental ❌
- resetTransform ✅
- restore ✅
- rotate ✅
- roundRect ❌
- save ✅
- scale ✅
- scrollPathIntoViewExperimental ❌
- setLineDash ✅
- setTransform ✅
  - Accept matrix object as parameter
- shadowBlur ❌
- shadowColor ❌
- shadowOffsetX ❌
- shadowOffsetY ❌
- stroke ✅
  - path parameter ✅
- strokeRect ✅
- strokeStyle ✅baisc
- strokeText ✅
- textAlign ❌
- textBaseline ❌
- textRenderingExperimental ❌
- transform ✅
- translate ✅
- wordSpacingExperimental ❌
