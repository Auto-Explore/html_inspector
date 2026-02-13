# html/semantics/forms/resetting-a-form/reset-form-event-realm.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/resetting-a-form/reset-form-event-realm.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>reset() event firing realm</title>
<link rel="help" href="https://html.spec.whatwg.org/#resetting-a-form">
<link rel="help" href="https://dom.spec.whatwg.org/#concept-event-fire">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe src="support/reset-form-event-realm.html"></iframe>
<iframe></iframe>

<script>
"use strict";

async_test(t => {
  window.onload = t.step_func_done(() => {
    const frame0Form  = frames[0].document.forms[0];
    const frame1Body = frames[1].document.body;

    frame1Body.appendChild(frame0Form);

    let resetCalled = false;
    frame0Form.onreset = t.step_func(ev => {
      resetCalled = true;

      const functionConstructorInEvRealm = ev.constructor.constructor;
      const functionConstructorInFormRealm = frame0Form.constructor.constructor;

      assert_equals(functionConstructorInEvRealm, functionConstructorInFormRealm,
        "the event must be created in the realm of the target");
    });

    frame0Form.reset();
    assert_true(resetCalled, "The reset event handler must have been called");
  });
}, "reset()'s event must be fired in the Realm of the target")
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
  "source_name": "html/semantics/forms/resetting-a-form/reset-form-event-realm.html"
}
```
