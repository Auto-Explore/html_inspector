# html/semantics/popovers/popover-appearance.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-appearance.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Popover element appearance</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<link rel="match" href="popover-appearance-ref.html">

<style>
[popover] {top: 100px; bottom: auto;}
[popover=""] {left: -300px}
[popover=auto] {left: -100px; }
[popover=manual] {left: 100px; }
[popover=invalid] {left: 300px; }
</style>

<p>There should be four popovers with similar appearance.</p>
<div popover>Blank
  <div popover=auto>Auto</div>
</div>
<div popover=manual>Manual</div>
<!-- This ensures unsupported popover values are treated as popover=manual -->
<div popover=invalid>Invalid</div>
<script>
  document.querySelectorAll('[popover]').forEach(p => p.showPopover());
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
        "byte_end": 783,
        "byte_start": 762,
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
  "source_name": "html/semantics/popovers/popover-appearance.html"
}
```
