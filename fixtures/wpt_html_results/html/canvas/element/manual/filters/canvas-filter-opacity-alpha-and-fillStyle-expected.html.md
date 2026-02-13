# html/canvas/element/manual/filters/canvas-filter-opacity-alpha-and-fillStyle-expected.html

Counts:
- errors: 3
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/canvas-filter-opacity-alpha-and-fillStyle-expected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>
  <div id="sq-1"></div>
  <div id="sq-2"></div>
  <div id="sq-3"></div>
  <div id="sq-4"></div>
</body>
<style>
  /*The expected behavior when setting the opacity through different methods
  is that the opacity of the resulting drawn element is the product of the opacity
  value set by each of the methods.*/
  div{
    width: 50px;
    height: 50px;
    position: absolute;
  }
  #sq-1{
    background-color: black;
    left:  18px;
    top:  18px;
    opacity: 0.125;
  }
  #sq-2{
    background-color: black;
    left:  78px;
    top:  18px;
    opacity: 0.03125;
  }
  #sq-3{
    background-color: black;
    left:  18px;
    top:  78px;
    opacity: 0.1875;
  }
  #sq-4{
    background-color: black;
    left:  78px;
    top:  78px;
    opacity: 0.016;
  }
</style>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “style”.",
      "severity": "Error",
      "span": {
        "byte_end": 134,
        "byte_start": 127,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 134,
        "byte_start": 127,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 786,
        "byte_start": 134,
        "col": 8,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 794,
        "byte_start": 786,
        "col": 1,
        "line": 41
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
  "source_name": "html/canvas/element/manual/filters/canvas-filter-opacity-alpha-and-fillStyle-expected.html"
}
```
