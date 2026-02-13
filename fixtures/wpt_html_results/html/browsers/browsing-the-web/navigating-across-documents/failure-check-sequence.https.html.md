# html/browsers/browsing-the-web/navigating-across-documents/failure-check-sequence.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/failure-check-sequence.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Sequence of the checks performed against a navigation response</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script>
'use strict';
const collect = (win) => {
  const report = new Promise((resolve) => {
    if (!win.ReportingObserver) {
      return;
    }

    const observer = new win.ReportingObserver(resolve);
    observer.observe();
  }).then((reports) => reports[0].type);
  // Although CSP also makes use of ReportingObserver, monitoring this event
  // allows the test to provide value to implementations that have not yet
  // integrated CSP and Reporting (as of the time of this writing, Firefox and
  // Safari).
  const cspViolation = new Promise((resolve) => {
    win.document.addEventListener('securitypolicyviolation', () => resolve('csp-violation'));
  });
  const halfASecond = new Promise((resolve) => setTimeout(() => resolve(null), 500));

  return Promise.race([report, cspViolation, halfASecond]);
};

const createWindow = (t, url) => {
  const win = open(url);
  t.add_cleanup(() => win.close());
  return new Promise((resolve) => win.onload = () => resolve(win));
};

promise_test(async (t) => {
  const win = await createWindow(t, '/common/blank.html?pipe=header(content-security-policy, frame-src none)');
  const iframe = win.document.createElement('iframe');
  iframe.src = '/common/blank.html?pipe=header(x-frame-options, deny)';
  win.document.body.appendChild(iframe);

  assert_equals(await collect(win), 'csp-violation');
}, 'CSP check precedes X-Frame-Options check');

promise_test(async (t) => {
  const win = await createWindow(t, '/common/blank.html?pipe=header(content-security-policy, frame-src none)|header(cross-origin-embedder-policy, require-corp)');
  const iframe = win.document.createElement('iframe');
  iframe.src = '/common/blank.html';
  win.document.body.appendChild(iframe);

  assert_equals(await collect(win), 'csp-violation');
}, 'CSP check precedes COEP check - CSP header first');

promise_test(async (t) => {
  const win = await createWindow(t, '/common/blank.html?pipe=header(cross-origin-embedder-policy, require-corp)|header(content-security-policy, frame-src none)');
  const iframe = win.document.createElement('iframe');
  iframe.src = '/common/blank.html';
  win.document.body.appendChild(iframe);

  assert_equals(await collect(win), 'csp-violation');
}, 'CSP check precedes COEP check - COEP header first');

promise_test(async (t) => {
  const win = await createWindow(t, '/common/blank.html?pipe=header(cross-origin-embedder-policy, require-corp)');
  const iframe = win.document.createElement('iframe');
  iframe.src = '/common/blank.html?pipe=header(x-frame-options, deny)';
  win.document.body.appendChild(iframe);

  assert_equals(await collect(win), 'coep');
}, 'COEP check precedes X-Frame-Options check');
</script>
</body>
</html>
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/failure-check-sequence.https.html"
}
```
