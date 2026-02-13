# html/semantics/popovers/popover-hidden-display.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-hidden-display.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel=author href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<link rel=match href="popover-hidden-display-ref.html">
<meta name=fuzzy content="0-1;0-15">

<div class=nottoplayer popover                           >This content should be visible and green</div>
<div class=nottoplayer popover=invalid style="top:100px;">This content should be visible and green</div>
<div class=toplayer    popover         style="top:200px;">This content should be visible and green</div>

<style>
  [popover] {
    display: block; /* This should make the popover visible */
    top: 0;
    margin:10px;
    width: 300px;
    height: 50px;
  }
  [popover].nottoplayer {
    background: green;
  }
  [popover].toplayer {
    background: red;
  }
  [popover].toplayer:popover-open {
    background: green;
  }
  [popover].nottoplayer:popover-open {
    background: red;
  }
</style>
<script>
  const toplayer = document.querySelectorAll('[popover].toplayer');
  if (toplayer.length !== 1)
    document.write('FAIL');
  toplayer[0].showPopover();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.popover.value.invalid",
      "message": "Bad value “invalid” for attribute “popover” on element “div”.",
      "severity": "Warning",
      "span": {
        "byte_end": 504,
        "byte_start": 446,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 664,
        "byte_start": 657,
        "col": 1,
        "line": 13
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
  "source_name": "html/semantics/popovers/popover-hidden-display.html"
}
```
