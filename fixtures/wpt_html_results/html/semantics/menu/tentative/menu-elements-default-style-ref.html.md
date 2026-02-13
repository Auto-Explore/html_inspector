# html/semantics/menu/tentative/menu-elements-default-style-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/menu-elements-default-style-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=stylesheet href=resources/menu-elements-styles.css>

<style>
body { background-color: orange; }
</style>

<div class="menubar">
  <div class="menuitem">Menubar item A</div>
  <div class="menuitem">Menubar item B</div>
  <div class="menuitem">Menubar item C</div>
</div>

<!-- A <br> here because menubars are inline(-flex) by default -->
<br>

<div class="menulist">
  <div class="menuitem">Menulist item A</div>
  <div class="menuitem">Menulist item B</div>
  <hr>
  <div class="menuitem">Menulist item C</div>
</div>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/menu/tentative/menu-elements-default-style-ref.html"
}
```
