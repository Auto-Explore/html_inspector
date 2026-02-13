# html/semantics/forms/the-select-element/customizable-select/select-popover-exit-animation-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-popover-exit-animation-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=stylesheet href="resources/customizable-select-styles.css">

<div>hover target</div>
<div class=customizable-select-button popovertarget=popover id=button>
  <span class=customizable-select-selectedcontent>option</span>
</div>
<div id=popover popover=auto anchor=button class=customizable-select-popover>
  <div class="customizable-select-option selected">option</div>
</div>

<script>
document.getElementById('popover').showPopover();
</script>

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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-popover-exit-animation-ref.html"
}
```
