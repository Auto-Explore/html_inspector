# html/semantics/scripting-1/the-script-element/moving-between-documents/ordering/parser-blocking.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/moving-between-documents/ordering/parser-blocking.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/C/#pending-parsing-blocking-script">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="helper.js"></script>
<body>
<script>
const t = async_test('Script elements (parser-blocking) still block the parser in the original Document after moved to another Document');
const start_time = performance.now();
const iframe = document.createElement('iframe');
document.body.appendChild(iframe);

t.step_timeout(() => {
  const script = document.querySelector('#to-be-moved');
  iframe.contentDocument.body.appendChild(script);
}, 1000);

let syncScriptEvaluated = false;

const onSyncScript = t.step_func(() => {
  syncScriptEvaluated = true;

  // The `#to-be-moved` script should block the parser and thus the sync
  // script after `#to-be-moved` should be delayed until `#to-be-moved` is
  // loaded (i.e. 3 seconds).
  // Here we expect the delay should be at least 2 seconds,
  // as the latency can be slightly less than 3 seconds due to preloading.
  assert_greater_than(performance.now() - start_time, 2000,
      'Parser should be blocked until script is loaded');
});

document.addEventListener('DOMContentLoaded', t.step_func_done(() => {
  assert_true(syncScriptEvaluated,
              'sync script should be evaluated before DOMContentLoaded');
  assert_greater_than(performance.now() - start_time, 2000,
      'DOMContentLoaded event should be delayed until script is loaded');
}));
</script>
<script id="to-be-moved" src="../../resources/throw.js?pipe=trickle(d3)"></script>
<script src="data:text/javascript,onSyncScript()"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/moving-between-documents/ordering/parser-blocking.html"
}
```
