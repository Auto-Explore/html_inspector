# html/semantics/embedded-content/the-iframe-element/iframe_navigate_javascript_url.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_navigate_javascript_url.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Navigate an iframe to a javascript: URL</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#process-the-iframe-attributes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
const tests = [
  // url, expected_load_event
  ['javascript:"foo"', true],
  ['javascript:1', false],
];
for (const [url, expected_load_event] of tests) {
  promise_test(async t => {
    const iframe = document.createElement('iframe');
    const loaded = new Promise(resolve => { iframe.onload = resolve; });
    document.body.append(iframe);
    await loaded;
    const secondLoad = new Promise((resolve, reject) => {
      iframe.onload = () => {
        expected_load_event ? resolve() : reject();
      };
    });
    const timeout = new Promise((resolve, reject) => {
      t.step_timeout(() => {
        expected_load_event ? reject() : resolve();
      }, 100);
    });
    iframe.contentWindow.location.href = url;
    await Promise.race([secondLoad, timeout]);
  }, `location.href = '${url}' should ${expected_load_event ? '' : 'not '}fire a load event`);
}
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_navigate_javascript_url.htm"
}
```
