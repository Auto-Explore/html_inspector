# html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-menu.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-menu.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>list owner is calculated to be narest ancestor menu if it exists</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#ordinal-value">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#list-owner">

<link rel="match" href="grouping-li-reftest-list-owner-menu-ref.html">

<style>
  li {
    list-style-type: decimal;
  }
</style>

<p>This test matches if the list displays similar to the following</p>

<pre>1. A
2. B
3. C
4. D
5. E
     1. F
     2. G
6. H
     1. I
     2. J
          1. K
          2. L</pre>

<hr>

<menu>
  <li>A</li>
  <li>B</li>
  <div>
    <li>C</li>
    <span>
      <li>D</li>
      <li>E</li>
    </span>
    <menu>
      <li>F</li>
      <li>G</li>
    </menu>
  </div>
  <li>H</li>
  <menu>
    <li>I</li>
    <li>
      J
      <menu>
        <li>K</li>
        <li>L</li>
      </menu>
    </li>
  </menu>
</menu>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “div” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 736,
        "byte_start": 731,
        "col": 3,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 745,
        "byte_start": 741,
        "col": 5,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 773,
        "byte_start": 769,
        "col": 7,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 790,
        "byte_start": 786,
        "col": 7,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 896,
        "byte_start": 890,
        "col": 3,
        "line": 48
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-menu.html"
}
```
