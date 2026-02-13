# html/canvas/element/manual/filters/tentative/canvas-filter-object-turbulence-expected.html

Counts:
- errors: 4
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/tentative/canvas-filter-object-turbulence-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<body>
  <svg style="display:none">>
    <filter id="base">
      <feTurbulence baseFrequency="0.03125"/>
    </filter>
    <filter id="base2d">
      <feTurbulence baseFrequency="0.03125, 0.125"/>
    </filter>
    <filter id="highFrequency">
      <feTurbulence baseFrequency="0.0625"/>
    </filter>
    <filter id="seed">
      <feTurbulence baseFrequency="0.03125" seed="100"/>
    </filter>
    <filter id="numOctaves">
      <feTurbulence baseFrequency="0.03125" numOctaves="2"/>
    </filter>
    <filter id="empty">
      <feTurbulence/>
    </filter>
    <filter id="fractalNoise">
      <feTurbulence baseFrequency="0.03125" type="fractalNoise"/>
    </filter>
    <filter id="stitchTiles">
      <feTurbulence baseFrequency="0.03125" stitchTiles="noStitch"/>
    </filter>
</body>
<script>
  testCases = document.getElementsByTagName("filter");
  for (tc of testCases) {
    const canvas = document.createElement("canvas");
    document.body.appendChild(canvas);
    const ctx = canvas.getContext("2d");
    ctx.filter = `url(#${tc.id})`;
    ctx.fillRect(0, 0, 1, 1);
  }
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 6,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 801,
        "byte_start": 793,
        "col": 1,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1085,
        "byte_start": 801,
        "col": 9,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1094,
        "byte_start": 1085,
        "col": 1,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/element/manual/filters/tentative/canvas-filter-object-turbulence-expected.html"
}
```
