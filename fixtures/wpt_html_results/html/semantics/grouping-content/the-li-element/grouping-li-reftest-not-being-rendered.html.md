# html/semantics/grouping-content/the-li-element/grouping-li-reftest-not-being-rendered.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-not-being-rendered.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>List items that are not being rendered do not participate in numbering</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#ordinal-value">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#list-owner">

<link rel="match" href="grouping-li-reftest-not-being-rendered-ref.html">

<p>This test matches if the list displays similar to the following</p>

<pre>1. A
2. B
3. D
4. E
5. G</pre>

<hr>

<ol>
  <li>A</li>
  <li>B</li>
  <li hidden>C</li>
  <li>D</li>
  <div>
    <li>E</li>
    <li hidden>F</li>
    <li>G</li>
  </div>
</ol>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 651,
        "byte_start": 647,
        "col": 5,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 673,
        "byte_start": 662,
        "col": 5,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 688,
        "byte_start": 684,
        "col": 5,
        "line": 28
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-not-being-rendered.html"
}
```
