# html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-resolution-order.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-resolution-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Promise rejection ordering</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#perform-a-microtask-checkpoint">
<body>
<p>A microtask checkpoint should notify about rejected promises. After
<a
href="https://html.spec.whatwg.org/multipage/webappapis.html#clean-up-after-running-script">cleaning
up after running script</a> involves running a microtask checkpoint. So the order of unhandledrejection
should occur before error2.
</p>

<script>
'use strict';
setup({ allow_uncaught_exception: true });

async_test(function(t) {
  const events = [];
  addEventListener('unhandledrejection', t.step_func(() => {
    events.push('unhandledrejection');
  }));

  function insertInvalidScript(id) {
    // Inserting <script> with an empty source schedules dispatching an 'error' event on
    // the dom manipulation task source.
    let script = document.createElement('script');
    script.setAttribute('src', '   ');
    script.addEventListener('error', t.step_func(() => {
      events.push(`error${id}`);

      // This will be the end of the test. Verify the results are correct.
      if (id == 2) {
        assert_array_equals(
          events,
          [
            'raf1',
            'resolve1',
            'raf2',
            'resolve2',
            'error1',
            'unhandledrejection',
            'error2'
          ]
        );
        t.done();
      }
    }));
    document.body.append(script);
  }

  requestAnimationFrame(t.step_func(() => {
    events.push('raf1');
    Promise.reject();
    Promise.resolve(0).then(t.step_func(() => {
      events.push('resolve1');
    }));
    insertInvalidScript(1);
  }));

  requestAnimationFrame(t.step_func(() => {
    events.push('raf2');
    Promise.resolve(0).then(t.step_func(() => {
      events.push('resolve2');
    }));
    insertInvalidScript(2);
  }));
});
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
  "source_name": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/promise-resolution-order.html"
}
```
