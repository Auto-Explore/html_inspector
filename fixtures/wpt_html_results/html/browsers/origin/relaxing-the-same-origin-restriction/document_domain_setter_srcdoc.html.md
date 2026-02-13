# html/browsers/origin/relaxing-the-same-origin-restriction/document_domain_setter_srcdoc.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/relaxing-the-same-origin-restriction/document_domain_setter_srcdoc.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<!-- SEKRITS! -->
<input id="sekrit" value="omg!">

<script>
  function postMessageToFrame(frame, message) {
    return new Promise(resolve => {
      var c = new MessageChannel();
      c.port1.onmessage = e => {
        resolve({ data: e.data, frame: frame })
      };
      frame.contentWindow.postMessage(message, '*', [c.port2]);
    });
  }

  function createFrame() {
    return new Promise(resolve => {
      var i = document.createElement('iframe');
      i.srcdoc = `
          <script>
            window.addEventListener('message', e => {
              if (e.data.domain !== undefined) {
                try {
                  document.domain = e.data.domain;
                  e.ports[0].postMessage('Done');
                } catch(error) {
                  e.ports[0].postMessage(error.name);
                }
              } else if (e.data == 'poke-at-parent') {
                try {
                  var sekrit = window.parent.document.body.querySelector('#sekrit').value;
                  e.ports[0].postMessage(sekrit);
                } catch(error) {
                  e.ports[0].postMessage(error.name);
                }
              }
            });
            window.parent.postMessage('Hi!', '*');
          </scr` + `ipt>`;
      window.addEventListener('message', m => {
        if (m.source == i.contentWindow)
          resolve(i);
      });
      document.body.appendChild(i);
    });
  }

  promise_test(t => {
    return createFrame()
      .then(f => postMessageToFrame(f, 'poke-at-parent'))
      .then(result => {
        assert_equals(result.data, document.querySelector('#sekrit').value);
        result.frame.remove();
      });
  }, "srcdoc can access with no 'document.domain' modification.");

  promise_test(t => {
    return createFrame()
      .then(f => postMessageToFrame(f, { domain: window.location.hostname }))
      .then(result => {
        assert_equals(result.data, 'Done');
        return postMessageToFrame(result.frame, 'poke-at-parent')
          .then(result => {
        assert_equals(result.data, document.querySelector('#sekrit').value);
            result.frame.remove();
          });
      });
  }, "srcdoc can access with valid 'document.domain'.");

  promise_test(t => {
    return createFrame()
      .then(f => {
        document.domain = window.location.hostname;
        return postMessageToFrame(f, 'poke-at-parent');
      })
      .then(result => {
        assert_equals(result.data, document.querySelector('#sekrit').value);
        result.frame.remove();
      });
  }, "srcdoc can access when parent modifies 'document.domain'.");
</script>
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
  "source_name": "html/browsers/origin/relaxing-the-same-origin-restriction/document_domain_setter_srcdoc.html"
}
```
