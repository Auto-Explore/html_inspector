# html/semantics/popovers/popover-light-dismiss-scroll-within.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-light-dismiss-scroll-within.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width,initial-scale=1.0">
<title>Popover light dismiss behavior when scrolled within</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/popover-utils.js"></script>

<style>
  [popover] {
    /* Position most popovers at the bottom-right, out of the way */
    inset:auto;
    bottom:0;
    right:0;
  }
  [popover]::backdrop {
    /* This should *not* affect anything: */
    pointer-events: auto;
  }
</style>

<div popover id=p>Inside popover
  <div style="height:2000px;background:lightgreen"></div>
  Bottom of popover
</div>
<button popovertarget=p>Popover</button>
<style>
  #p {
    width: 300px;
    height: 300px;
    overflow-y: scroll;
  }
</style>
<script>
  const popover = document.querySelector('#p');
  promise_test(async () => {
    popover.showPopover();
    assert_equals(popover.scrollTop,0,'popover should start non-scrolled');
    await new test_driver.Actions()
       .scroll(0, 0, 0, 50, {origin: popover})
       .send();
    await waitForRender();
    assert_true(popover.matches(':popover-open'),'popover should stay open');
    assert_equals(popover.scrollTop,50,'popover should be scrolled');
    popover.hidePopover();
  },'Scrolling within a popover should not close the popover');
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
        "byte_end": 1046,
        "byte_start": 1039,
        "col": 1,
        "line": 32
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
  "source_name": "html/semantics/popovers/popover-light-dismiss-scroll-within.html"
}
```
