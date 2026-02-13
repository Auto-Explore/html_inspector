# html/rendering/non-replaced-elements/lists/li-type-supported-ref.html

Counts:
- errors: 0
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/li-type-supported-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>li@type: supported types</title>
<style>
  .decimal { list-style-type: decimal; }
  .lower-alpha { list-style-type: lower-alpha; }
  .upper-alpha { list-style-type: upper-alpha; }
  .lower-roman { list-style-type: lower-roman; }
  .upper-roman { list-style-type: upper-roman; }
  .disc { list-style-type: disc; }
  .circle { list-style-type: circle; }
  .square { list-style-type: square; }
  .none { list-style-type: none; }
</style>
<li class="decimal">first item</li>
<li class="lower-alpha">second item</li>
<li class="upper-alpha">third item</li>
<li class="lower-roman">fourth item</li>
<li class="upper-roman">fifth item</li>
<li class="disc">sixth item</li>
<li class="circle">seventh item</li>
<li class="square">eighth item</li>
<li class="none">ninth item</li>
<ol>
  <li class="decimal">first ordered item</li>
  <li class="lower-alpha">second ordered item</li>
  <li class="upper-alpha">third ordered item</li>
  <li class="lower-roman">fourth ordered item</li>
  <li class="upper-roman">fifth ordered item</li>
  <li class="disc">sixth ordered item</li>
  <li class="circle">seventh ordered item</li>
  <li class="square">eighth ordered item</li>
  <li class="none">ninth ordered item</li>
</ol>
<ul>
  <li class="decimal">first unordered item</li>
  <li class="lower-alpha">second unordered item</li>
  <li class="upper-alpha">third unordered item</li>
  <li class="lower-roman">fourth unordered item</li>
  <li class="upper-roman">fifth unordered item</li>
  <li class="disc">sixth unordered item</li>
  <li class="circle">seventh unordered item</li>
  <li class="square">eighth unordered item</li>
  <li class="none">ninth unordered item</li>
</ul>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 499,
        "byte_start": 479,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 539,
        "byte_start": 515,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 580,
        "byte_start": 556,
        "col": 1,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 620,
        "byte_start": 596,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 661,
        "byte_start": 637,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 694,
        "byte_start": 677,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 729,
        "byte_start": 710,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 766,
        "byte_start": 747,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 800,
        "byte_start": 783,
        "col": 1,
        "line": 23
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
  "source_name": "html/rendering/non-replaced-elements/lists/li-type-supported-ref.html"
}
```
