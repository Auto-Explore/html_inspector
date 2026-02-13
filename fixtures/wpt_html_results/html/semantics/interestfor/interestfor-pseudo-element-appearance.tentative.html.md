# html/semantics/interestfor/interestfor-pseudo-element-appearance.tentative.html

Counts:
- errors: 1
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-pseudo-element-appearance.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<link rel="match" href="interestfor-pseudo-element-appearance-ref.html">

<div class=example>
  <button interestfor=target><span>Button</span></button>
</div>
<div class=example>
  <a href=# interestfor=target><span>Link</span></a>
</div>
<div class=example>
  <img src="/images/blue.png" usemap="#map1" width=40 height=40>
  <map id="map1"><area href="/" shape="default" interestfor=target></map>
</div>

<div id=target></div>

<style>
  [interestfor]::before {
    content: "::before";
    border: 1px solid green;
  }
  [interestfor]::interest-hint {
    content: "::interest-hint";
    font-family: arial;
    font-size: 13px;
  }
  [interestfor]::after {
    content: "::after";
    border: 1px solid green;
  }
  /* Test convenience: */
  .example {
    display:block;
    width: 400px;
    height: 50px;
  }
  [interestfor]>span {
    border: 1px solid black;
  }
</style>
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
        "byte_end": 502,
        "byte_start": 440,
        "col": 3,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 615,
        "byte_start": 608,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “map1”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 502,
        "byte_start": 440,
        "col": 3,
        "line": 14
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
  "source_name": "html/semantics/interestfor/interestfor-pseudo-element-appearance.tentative.html"
}
```
