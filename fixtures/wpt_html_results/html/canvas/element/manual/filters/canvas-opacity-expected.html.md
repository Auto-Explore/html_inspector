# html/canvas/element/manual/filters/canvas-opacity-expected.html

Counts:
- errors: 3
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/canvas-opacity-expected.html",
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
  <div id="sq-5"></div>
  <div id="sq-6"></div>
  <div id="sq-7"></div>
  <div id="sq-8"></div>
</body>
<style>
  div{
    width: 50px;
    height: 50px;
    position: absolute;
  }
  #sq-1{
    background-color: teal;
    left:  18px;
    top:  18px;
    opacity: 0.75;
  }
  #sq-2{
    background-color: magenta;
    left:  78px;
    top:  18px;
    opacity: 0.5;
  }
  #sq-3{
    background-color: orange;
    left:  18px;
    top:  78px;
    opacity: 0.25;
  }
  #sq-4{
    background-color: chocolate;
    left:  78px;
    top:  78px;
    opacity: 0;
  }
  #sq-5{
    background-color: cyan;
    left:  18px;
    top:  158px;
    opacity: 0.8;
  }
  #sq-6{
    background-color: red;
    left:  28px;
    top:  158px;
    opacity: 0.6;
  }
  #sq-7{
    background-color: yellow;
    left:  38px;
    top:  158px;
    opacity: 0.4;
  }
  #sq-8{
    background-color: green;
    left:  48px;
    top:  158px;
    opacity: 0.2;
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
        "byte_end": 230,
        "byte_start": 223,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 230,
        "byte_start": 223,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1053,
        "byte_start": 230,
        "col": 8,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1061,
        "byte_start": 1053,
        "col": 1,
        "line": 66
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
  "source_name": "html/canvas/element/manual/filters/canvas-opacity-expected.html"
}
```
