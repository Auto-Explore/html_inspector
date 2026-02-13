# html/semantics/embedded-content/the-area-element/area-processing.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-area-element/area-processing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>HTMLAreaElement processing</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
 body { margin: 0 }
</style>
<img src=/images/threecolors.png usemap=#x id=img width=300 height=300>
<map name=x><area id=area></map>
<script src=support/hit-test.js></script>
<script>
var tests = [
  {desc: 'too few numbers', shape: 'rect', coords: "2,2,10", hit: hitNone},
  {desc: 'negative', shape: 'rect', coords: "-10,-10,10,10", hit: [[area, 1, 1], [img, 299, 299]]},
  {desc: 'empty string', shape: 'rect', coords: "", hit: hitNone},
  {desc: 'omitted coords', shape: 'rect', coords: null, hit: hitNone},
  {desc: 'first > third', shape: 'rect', coords: "10,2,2,10", hit: hitRect},
  {desc: 'second > fourth', shape: 'rect', coords: "2,10,10,2", hit: hitRect},
  {desc: 'first > third, second > fourth', shape: 'rect', coords: "10,10,2,2", hit: hitRect},

  {desc: 'negative', shape: 'default', coords: "-10,-10,-10,-10", hit: hitAll},

  {desc: 'too few numbers', shape: 'circle', coords: "20,40", hit: hitNone},
  {desc: 'negative radius', shape: 'circle', coords: "20,40,-10", hit: hitNone},
  {desc: 'zero radius', shape: 'circle', coords: "20,40,0", hit: hitNone},

  {desc: 'too few numbers', shape: 'poly', coords: "100,100,120,100,100", hit: hitNone},
  {desc: 'one too many numbers', shape: 'poly', coords: "100,100,120,100,100,120,300", hit: hitPoly},
  {desc: 'even-odd rule', shape: 'poly', coords: "100,100,200,100,100,200,150,50,200,200", hit: hitStar},
];
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 289,
        "byte_start": 218,
        "col": 1,
        "line": 9
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
  "source_name": "html/semantics/embedded-content/the-area-element/area-processing.html"
}
```
