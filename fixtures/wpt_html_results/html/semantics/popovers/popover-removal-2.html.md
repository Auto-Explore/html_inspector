# html/semantics/popovers/popover-removal-2.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-removal-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Popover document removal behavior</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe id=frame1 srcdoc="<div popover id=popover>Popover</div>"></iframe>
<iframe id=frame2></iframe>

<script>
  window.onload = () => {
    test(t => {
      const frame1Doc = document.getElementById('frame1').contentDocument;
      const frame2Doc = document.getElementById('frame2').contentDocument;
      const popover = frame1Doc.querySelector('[popover]');
      assert_true(!!popover);
      assert_false(popover.matches(':popover-open'));
      popover.showPopover();
      assert_true(popover.matches(':popover-open'));
      frame2Doc.body.appendChild(popover);
      assert_false(popover.matches(':popover-open'));
      popover.showPopover();
      assert_true(popover.matches(':popover-open'));
    }, 'Moving popover between documents shouldn\'t cause issues');
  };
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/popovers/popover-removal-2.html"
}
```
