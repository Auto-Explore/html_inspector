# html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-parent-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-parent-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>list owner is calculated to be the parent if there is no ancestor ul/ol/menu</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#ordinal-value">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#list-owner">

<style>
  li {
    list-style-type: decimal;
    list-style-position: inside;
  }

  ol {
    padding: 50px;
    margin: 0;
  }
</style>

<p>This test matches if the list displays similar to the following</p>

<pre>1. A
2. B
1. C
1. D
       1. E
3. F</pre>

<hr>

<ol>
  <li value="1">A</li>
  <li value="2">B</li>
  <li value="1">C</li>
  <li value="1">D</li>
  <blockquote>
    <li value="1">E</li>
  </blockquote>
  <li value="3">F</li>
</ol>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “blockquote” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 782,
        "byte_start": 768,
        "col": 5,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 782,
        "byte_start": 768,
        "col": 5,
        "line": 37
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-parent-ref.html"
}
```
