# html/semantics/embedded-content/the-area-element/area-shape.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-area-element/area-shape.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>HTMLAreaElement shape</title>
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
  {desc: 'missing value default', shape: null, coords: "2,2,10,10", hit: hitRect},
  {desc: 'missing value default', shape: null, coords: "20,40,10", hit: hitNone},
  {desc: 'missing value default', shape: null, coords: null, hit: hitNone},
  {desc: 'invalid value default', shape: 'foobar invalid', coords: "2,2,10,10", hit: hitRect},
  {desc: 'invalid value default', shape: '', coords: "2,2,10,10", hit: hitRect},

  {desc: 'empty string', shape: 'default', coords: "", hit: hitAll},
  {desc: 'omitted coords', shape: 'DEFAULT', coords: null, hit: hitAll},

  {desc: 'simple', shape: 'rect', coords: "2,2,10,10", hit: hitRect},
  {desc: 'simple', shape: 'rectangle', coords: "2,2,10,10", hit: hitRect},

  {desc: 'simple', shape: 'circle', coords: "20,40,10", hit: hitCircle},
  {desc: 'simple', shape: 'circ', coords: "20,40,10", hit: hitCircle},
  {desc: 'simple', shape: 'CIRCLE', coords: "20,40,10", hit: hitCircle},
  {desc: 'simple', shape: 'CIRC', coords: "20,40,10", hit: hitCircle},
  {desc: 'LATIN CAPITAL LETTER I WITH DOT ABOVE', shape: 'C\u0130RCLE', coords: "20,40,10", hit: hitNone},
  {desc: 'LATIN SMALL LETTER DOTLESS I', shape: 'c\u0131rcle', coords: "20,40,10", hit: hitNone},

  {desc: 'simple', shape: 'poly', coords: "100,100,120,100,100,120", hit: hitPoly},
  {desc: 'simple', shape: 'polygon', coords: "100,100,120,100,100,120", hit: hitPoly},
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
        "byte_end": 284,
        "byte_start": 213,
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
  "source_name": "html/semantics/embedded-content/the-area-element/area-shape.html"
}
```
