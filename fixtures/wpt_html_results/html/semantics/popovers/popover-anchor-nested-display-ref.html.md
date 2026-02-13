# html/semantics/popovers/popover-anchor-nested-display-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-anchor-nested-display-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel=author href="mailto:xiaochengh@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">

<button id=main-menu-button>Show menu</button>

<div id=main-menu>
  <div>Foo</div>
  <button id=nested-menu-button>
    Show nested menu
  </button>
  <div>Bar</div>
</div>

<div id=nested-menu>
  Baz
</div>

<style>
#main-menu-button {
  position: absolute;
  top: 200px;
  left: 100px;
  width: 100px;
}

#main-menu {
  position: absolute;
  top: 200px;;
  left: 200px;
  width: 150px;
  line-height: 20px;
}

#nested-menu-button {
  width: 100%;
}

#nested-menu {
  position: absolute;
  top: 220px;
  left: 350px;
}

[popover] {
  border: 0;
  margin: 0;
  padding: 0;
}
</style>

<script>
document.getElementById('main-menu-button').click();
document.getElementById('nested-menu-button').click();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 469,
        "byte_start": 462,
        "col": 1,
        "line": 21
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
  "source_name": "html/semantics/popovers/popover-anchor-nested-display-ref.html"
}
```
