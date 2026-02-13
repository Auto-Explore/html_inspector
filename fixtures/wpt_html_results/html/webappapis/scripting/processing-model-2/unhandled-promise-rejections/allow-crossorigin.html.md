# html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/allow-crossorigin.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/allow-crossorigin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/cors/support.js?pipe=sub"></script>
<link rel="help" href="https://html.spec.whatwg.org/#unhandled-promise-rejections">
<link rel="help" href="https://html.spec.whatwg.org/#muted-errors">

<body>
<script>
'use strict';
setup({
  allow_uncaught_exception: true
});

async_test(function(t) {
  addEventListener('unhandledrejection', t.step_func(function(e) {
    assert_equals(e.reason, 42, 'reason should be the one given by the script');
    t.done();
  }));
}, 'Promise rejection event should be received for the cross-origin CORS script');

(function() {
  var scriptEl = document.createElement('script');
  scriptEl.src = CROSSDOMAIN + 'support/promise-access-control.py?allow=true';
  scriptEl.crossOrigin = 'anonymous';
  document.body.appendChild(scriptEl);
}());
</script>
</body>
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
  "source_name": "html/webappapis/scripting/processing-model-2/unhandled-promise-rejections/allow-crossorigin.html"
}
```
