# html/semantics/interestfor/interestfor-keyboard-invalidation-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-keyboard-invalidation-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<link rel="author" href="mailto:masonf@chromium.org">

<button>Button</button>
<button>Button</button>
<button id=hasinterest>Button</button>
<button class=otherselector>Button</button>
<button class=otherselector>Button</button>
<div popover id=target>Target</div>

<style>
  #hasinterest {
    background-color: purple;
  }
  .otherselector {
    background-color: green;
  }
  #target {
    background-color: yellow;
    inset:auto;
    top:50px;
    left:0;
  }
</style>

<script>
  target.showPopover();
  hasinterest.focus();
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
        "byte_end": 315,
        "byte_start": 308,
        "col": 1,
        "line": 12
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
  "source_name": "html/semantics/interestfor/interestfor-keyboard-invalidation-ref.html"
}
```
