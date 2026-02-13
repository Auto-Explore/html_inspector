# html/semantics/popovers/popover-anchor-nested-display.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-anchor-nested-display.tentative.html",
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
<link rel=match href="popover-anchor-nested-display-ref.html">

<button id=main-menu-button popovertarget=main-menu>Show menu</button>

<div id=main-menu popover anchor=main-menu-button>
  <div>Foo</div>
  <button id=nested-menu-button popovertarget=nested-menu>
    Show nested menu
  </button>
  <div>Bar</div>
</div>

<div id=nested-menu popover anchor=nested-menu-button>
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
  top: anchor(top);
  left: anchor(right);
  width: 150px;
  line-height: 20px;
}

#nested-menu-button {
  width: 100%;
}

#nested-menu {
  top: anchor(top);
  left: anchor(right);
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
        "byte_end": 648,
        "byte_start": 641,
        "col": 1,
        "line": 22
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
  "source_name": "html/semantics/popovers/popover-anchor-nested-display.tentative.html"
}
```
