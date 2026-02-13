# html/semantics/scripting-1/the-script-element/json-module/credentials.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/credentials.sub.html",
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

<script>
document.cookie = 'milk=1';

const setCookiePromise = fetch(
    'http://{{domains[www2]}}:{{ports[http][0]}}/cookies/resources/set-cookie.py?name=milk&path=/html/semantics/scripting-1/the-script-element/json-module/',
    {
      mode: 'no-cors',
      credentials: 'include',
    });

const windowLoadPromise = new Promise(resolve => {
  window.addEventListener('load', () => {
    resolve();
  });
});

promise_test(t => {
  const iframe = document.createElement('iframe');

  return Promise.all([setCookiePromise, windowLoadPromise]).then(() => {
    const messagePromise = new Promise(resolve => {
      window.addEventListener('message', event => {
        resolve();
      });
    });

    iframe.src = 'credentials-iframe.sub.html';
    document.body.appendChild(iframe);

    return messagePromise;
  }).then(() => {
    const w = iframe.contentWindow;

    assert_equals(w.sameOriginNoneDescendant, true,
                  'Descendant JSON modules should be loaded with the credentials when the crossOrigin attribute is not specified and the target is same-origin');
    assert_equals(w.sameOriginAnonymousDescendant, true,
                  'Descendant JSON modules should be loaded with the credentials when the crossOrigin attribute is specified with "anonymous" as its value and the target is same-origin');
    assert_equals(w.sameOriginUseCredentialsDescendant, true,
                  'Descendant JSON modules should be loaded with the credentials when the crossOrigin attribute is specified with "use-credentials" as its value and the target is same-origin');
    assert_equals(w.crossOriginNoneDescendant, false,
                  'Descendant JSON modules should not be loaded with the credentials when the crossOrigin attribute is not specified and the target is cross-origin');
    assert_equals(w.crossOriginAnonymousDescendant, false,
                  'Descendant JSON modules should not be loaded with the credentials when the crossOrigin attribute is specified with "anonymous" as its value and the target is cross-origin');
    assert_equals(w.crossOriginUseCredentialsDescendant, true,
                  'Descendant JSON modules should be loaded with the credentials when the crossOrigin attribute is specified with "use-credentials" as its value and the target is cross-origin');
});
}, 'JSON Modules should be loaded with or without the credentials based on the same-origin-ness and the crossOrigin attribute');
</script>
<body>
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
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/credentials.sub.html"
}
```
