# html/semantics/scripting-1/the-script-element/module/credentials.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/credentials.sub.html",
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
document.cookie = 'same=1';

const setCookiePromise = fetch(
    'http://{{domains[www2]}}:{{ports[http][0]}}/cookies/resources/set-cookie.py?name=cross&path=/html/semantics/scripting-1/the-script-element/module/',
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

    iframe.src = 'resources/credentials-iframe.sub.html';
    document.body.appendChild(iframe);

    return messagePromise;
  }).then(() => {
    const w = iframe.contentWindow;

    assert_equals(w.sameOriginNone, 'found',
                  'Modules should be loaded with the credentials when the crossOrigin attribute is not specified and the target is same-origin');
    assert_equals(w.sameOriginAnonymous, 'found',
                  'Modules should be loaded with the credentials when the crossOrigin attribute is specified with "anonymous" as its value and the target is same-origin');
    assert_equals(w.sameOriginUseCredentials, 'found',
                  'Modules should be loaded with the credentials when the crossOrigin attribute is specified with "use-credentials" as its value and the target is same-origin');
    assert_equals(w.crossOriginNone, 'not found',
                  'Modules should not be loaded with the credentials when the crossOrigin attribute is not specified and the target is cross-origin');
    assert_equals(w.crossOriginAnonymous, 'not found',
                  'Modules should not be loaded with the credentials when the crossOrigin attribute is specified with "anonymous" as its value and the target is cross-origin');
    assert_equals(w.crossOriginUseCredentials, 'found',
                  'Modules should be loaded with the credentials when the crossOrigin attribute is specified with "use-credentials" as its value and the target is cross-origin');

    assert_equals(w.sameOriginNoneDescendant, 'found',
                  'Descendant modules should be loaded with the credentials when the crossOrigin attribute is not specified and the target is same-origin');
    assert_equals(w.sameOriginAnonymousDescendant, 'found',
                  'Descendant modules should be loaded with the credentials when the crossOrigin attribute is specified with "anonymous" as its value and the target is same-origin');
    assert_equals(w.sameOriginUseCredentialsDescendant, 'found',
                  'Descendant modules should be loaded with the credentials when the crossOrigin attribute is specified with "use-credentials" as its value and the target is same-origin');
    assert_equals(w.crossOriginNoneDescendant, 'not found',
                  'Descendant modules should not be loaded with the credentials when the crossOrigin attribute is not specified and the target is cross-origin');
    assert_equals(w.crossOriginAnonymousDescendant, 'not found',
                  'Descendant modules should not be loaded with the credentials when the crossOrigin attribute is specified with "anonymous" as its value and the target is cross-origin');
    assert_equals(w.crossOriginUseCredentialsDescendant, 'found',
                  'Descendant modules should be loaded with the credentials when the crossOrigin attribute is specified with "use-credentials" as its value and the target is cross-origin');
});
}, 'Modules should be loaded with or without the credentials based on the same-origin-ness and the crossOrigin attribute');
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/credentials.sub.html"
}
```
