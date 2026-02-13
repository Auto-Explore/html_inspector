# html/semantics/popovers/popover-backdrop-appearance-ref.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-backdrop-appearance-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Popover ::backdrop pseudo element appearance</title>
<link rel="stylesheet" href="resources/popover-styles.css">

<style>
#bottom { top: 70px; left: 70px; }
#middle { top: 120px; left: 120px; }
#top { top: 170px; left: 170px; }
.fake-popover-backdrop {
  height: 200px;
  width: 200px;
}
#bottom-backdrop {
    top: 50px;
    left: 50px;
    background-color: rgb(0, 50, 0);
}
#middle-backdrop {
    top: 100px;
    left: 100px;
    background-color: rgb(0, 130, 0);
}
#top-backdrop {
    top: 150px;
    left: 150px;
    background-color: rgb(0, 210, 0);
}
.fake-popover {
  margin:0;
}
</style>
<p>Test for [popover]::backdrop presence and stacking order. The test passes
  if there are 3 stacked boxes, with the brightest green on top.</p>
<div popover id=bottom>Bottom
  <div popover id=middle>Middle
    <div popover=manual id=top>Top</div>
  </div>
</div>
<div id="bottom-backdrop" class="fake-popover-backdrop"></div>
<div id="bottom" class="fake-popover">Bottom</div>
<div id="middle-backdrop" class="fake-popover-backdrop"></div>
<div id="middle" class="fake-popover">Middle</div>
<div id="top-backdrop" class="fake-popover-backdrop"></div>
<div id="top" class="fake-popover">Top</div>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “bottom”.",
      "severity": "Error",
      "span": {
        "byte_end": 1011,
        "byte_start": 973,
        "col": 1,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “middle”.",
      "severity": "Error",
      "span": {
        "byte_end": 1125,
        "byte_start": 1087,
        "col": 1,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “top”.",
      "severity": "Error",
      "span": {
        "byte_end": 1233,
        "byte_start": 1198,
        "col": 1,
        "line": 45
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
  "source_name": "html/semantics/popovers/popover-backdrop-appearance-ref.html"
}
```
