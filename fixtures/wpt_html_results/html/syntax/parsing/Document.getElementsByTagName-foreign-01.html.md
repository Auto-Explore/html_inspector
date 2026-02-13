# html/syntax/parsing/Document.getElementsByTagName-foreign-01.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/Document.getElementsByTagName-foreign-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>document.getElementsByTagName and foreign parser-inserted
elements</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://dom.spec.whatwg.org/#dom-document-getelementsbytagname">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#parsing">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<svg>
<altglyph/>
  <altglyphdef/>
  <altglyphitem/>
  <animatecolor/>
  <animatemotion/>
  <animatetransform/>
  <clippath/>
  <feblend/>
  <fecolormatrix/>
  <fecomponenttransfer/>
  <fecomposite/>
  <feconvolvematrix/>
  <fediffuselighting/>
  <fedisplacementmap/>
  <fedistantlight/>
  <feflood/>
  <fefunca/>
  <fefuncb/>
  <fefuncg/>
  <fefuncr/>
  <fegaussianblur/>
  <feimage/>
  <femerge/>
  <femergenode/>
  <femorphology/>
  <feoffset/>
  <fepointlight/>
  <fespecularlighting/>
  <fespotlight/>
  <fetile/>
  <feturbulence/>
  <foreignobject/>
  <glyphref/>
  <lineargradient/>
  <radialgradient/>
  <textpath/>
  <ALTGLYPH/>
  <ALTGLYPHDEF/>
  <ALTGLYPHITEM/>
  <ANIMATECOLOR/>
  <ANIMATEMOTION/>
  <ANIMATETRANSFORM/>
  <CLIPPATH/>
  <FEBLEND/>
  <FECOLORMATRIX/>
  <FECOMPONENTTRANSFER/>
  <FECOMPOSITE/>
  <FECONVOLVEMATRIX/>
  <FEDIFFUSELIGHTING/>
  <FEDISPLACEMENTMAP/>
  <FEDISTANTLIGHT/>
  <FEFLOOD/>
  <FEFUNCA/>
  <FEFUNCB/>
  <FEFUNCG/>
  <FEFUNCR/>
  <FEGAUSSIANBLUR/>
  <FEIMAGE/>
  <FEMERGE/>
  <FEMERGENODE/>
  <FEMORPHOLOGY/>
  <FEOFFSET/>
  <FEPOINTLIGHT/>
  <FESPECULARLIGHTING/>
  <FESPOTLIGHT/>
  <FETILE/>
  <FETURBULENCE/>
  <FOREIGNOBJECT/>
  <GLYPHREF/>
  <LINEARGRADIENT/>
  <RADIALGRADIENT/>
  <TEXTPATH/>
</svg>
<script>
var elements = [
  "altGlyph",
  "altGlyphDef",
  "altGlyphItem",
  "animateColor",
  "animateMotion",
  "animateTransform",
  "clipPath",
  "feBlend",
  "feColorMatrix",
  "feComponentTransfer",
  "feComposite",
  "feConvolveMatrix",
  "feDiffuseLighting",
  "feDisplacementMap",
  "feDistantLight",
  "feFlood",
  "feFuncA",
  "feFuncB",
  "feFuncG",
  "feFuncR",
  "feGaussianBlur",
  "feImage",
  "feMerge",
  "feMergeNode",
  "feMorphology",
  "feOffset",
  "fePointLight",
  "feSpecularLighting",
  "feSpotLight",
  "feTile",
  "feTurbulence",
  "foreignObject",
  "glyphRef",
  "linearGradient",
  "radialGradient",
  "textPath"];
</script>
</div>
<script>
var SVG = "http://www.w3.org/2000/svg";
function t(el) {
  assert_equals(document.getElementsByTagName(el).length, 2);
  assert_equals(document.getElementsByTagName(el.toUpperCase()).length, 0);
  assert_equals(document.getElementsByTagName(el.toLowerCase()).length, 0);
  assert_equals(document.getElementsByTagNameNS(SVG, el).length, 2);
  assert_equals(document.getElementsByTagNameNS(SVG, el.toUpperCase()).length, 0);
  assert_equals(document.getElementsByTagNameNS(SVG, el.toLowerCase()).length, 0);
}
test(function() {
  var tests = [];
  assert_equals(document.getElementsByTagName('svg').length, 1);
  for (var i = 0, il = elements.length; i < il; ++i) {
    tests.push(["Testing " + elements[i], elements[i]]);
  }
  generate_tests(t, tests);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.svg.element.feconvolvematrix.missing_order",
      "message": "Element “feConvolveMatrix” is missing required attribute “order”.",
      "severity": "Warning",
      "span": {
        "byte_end": 689,
        "byte_start": 670,
        "col": 3,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.svg.element.feconvolvematrix.missing_order",
      "message": "Element “feConvolveMatrix” is missing required attribute “order”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1309,
        "byte_start": 1290,
        "col": 3,
        "line": 59
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/syntax/parsing/Document.getElementsByTagName-foreign-01.html"
}
```
