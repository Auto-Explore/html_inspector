# html/semantics/popovers/popover-open-in-beforetoggle.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-open-in-beforetoggle.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Popover beforetoggle event opening new popovers</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/indices.html#event-beforetoggle">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/popover-utils.js"></script>

<div popover id=p1>Popover 1
  <div popover id=p2>Popover 2
    <div popover id=p3>Popover 3</div>
  </div>
</div>
<div id=outside>Outside</div>
<dialog id=dialog>Dialog</dialog>

<script>
  function getSignal(t) {
    const controller = new AbortController();
    t.add_cleanup(() => controller.abort());
    return controller.signal;
  }

  test((t) => {
    p1.showPopover();
    p1.addEventListener('beforetoggle',() => p2.showPopover(),{signal: getSignal(t)});
    p1.hidePopover(); // Potential crash here
    assert_false(p1.matches(':popover-open'));
    assert_false(p2.matches(':popover-open'));
  },'Open popover from closing beforetoggle event');

  test((t) => {
    p1.showPopover();
    p1.addEventListener('beforetoggle',() => p2.showPopover(),{signal: getSignal(t)});
    p2.addEventListener('beforetoggle',() => p3.showPopover(),{signal: getSignal(t)});
    p1.hidePopover(); // Potential crash here
    assert_false(p1.matches(':popover-open'));
    assert_false(p2.matches(':popover-open'));
    assert_false(p3.matches(':popover-open'));
  },'Open double-nested popovers from closing beforetoggle event');

  test(t => {
    p1.showPopover();
    p1.addEventListener('beforetoggle',() => p2.showPopover(),{signal: getSignal(t)});
    p2.addEventListener('beforetoggle',() => p3.showPopover(),{signal: getSignal(t)});
    dialog.showModal(); // Potential crash here
    assert_false(p1.matches(':popover-open'));
    assert_false(p2.matches(':popover-open'));
    assert_false(p3.matches(':popover-open'));
    dialog.close();
  },'Open double-nested popovers from closing beforetoggle event, dialog open');

  promise_test(async t => {
    p1.showPopover();
    p1.addEventListener('beforetoggle',() => p2.showPopover(),{signal: getSignal(t)});
    p2.addEventListener('beforetoggle',() => p3.showPopover(),{signal: getSignal(t)});
    await clickOn(outside); // Potential crash here
    assert_false(p1.matches(':popover-open'));
    assert_false(p2.matches(':popover-open'));
    assert_false(p3.matches(':popover-open'));
  },'Open double-nested popovers from closing beforetoggle event, light dismiss');
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
  "source_name": "html/semantics/popovers/popover-open-in-beforetoggle.html"
}
```
