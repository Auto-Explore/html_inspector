# html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_in_src.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_in_src.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>javascript: URL in iframe src</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#process-the-iframe-attributes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
const tests = [
  // desc, url, expected_textContent
  ['string', 'javascript:"foo"', 'foo'],
  ['empty string', 'javascript:""', ''],
  ['String object', 'javascript:new String("foo")', ''],
  ['undefined', 'javascript:void(0)', ''],
  ['number', 'javascript:1', ''],
  ['boolean', 'javascript:true', ''],
  ['null', 'javascript:null', ''],
  ['global', 'javascript:window', ''],
  ['host object', 'javascript:document', ''],
  ['function', 'javascript:(() => { return function() {}; })()', ''],
  ['regexp', 'javascript:/foo/', ''],
  ['array', 'javascript:["foo"]', ''],
  ['object', 'javascript:{"foo": "bar"}', ''],
  ['ArrayBuffer', 'javascript:new ArrayBuffer(8)', ''],
  ['TypeError', 'javascript:new TypeError("foo")', ''],
];
for (const [desc, url, expected_textContent] of tests) {
  async_test(t => {
    const iframe = document.createElement('iframe');
    iframe.src = url;
    iframe.hidden = true;
    let load_events = 0;
    iframe.onload = t.step_func(() => {
      load_events++;
      assert_equals(iframe.contentDocument.URL, 'about:blank');
      assert_equals(iframe.contentDocument.body.textContent, expected_textContent);
      assert_equals(load_events, 1);
      if (load_events === 1) {
        t.step_timeout(() => {
          t.done();
        }, 100);
      }
    });
    document.body.append(iframe);
  }, `${desc}: ${url}`);
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_in_src.htm"
}
```
