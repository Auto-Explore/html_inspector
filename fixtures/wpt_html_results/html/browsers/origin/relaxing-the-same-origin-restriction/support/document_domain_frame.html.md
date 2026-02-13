# html/browsers/origin/relaxing-the-same-origin-restriction/support/document_domain_frame.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/relaxing-the-same-origin-restriction/support/document_domain_frame.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script>
  let cache = window;
  // "foo" needs to be a var so it's a property on the global.
  var foo = 'Reachable 1';
  // "bar" needs to be a var so it's a property on the global.
  var bar = { foo: 'Reachable 2' };
  location.foo = 'Reachable 3';
  document.foo = 'Reachable 4';
  window.addEventListener('message', e => {
    if (e.data.domain !== undefined) {
      try {
        document.domain = e.data.domain;
        e.ports[0].postMessage('Done');
      } catch(error) {
        e.ports[0].postMessage(error.name);
      }
    } else if (e.data['poke-at-sibling'] !== undefined) {
      try {
        var sekrit = parent[e.data['poke-at-sibling']].document.body.querySelector('#sekrit').value;
        e.ports[0].postMessage(sekrit);
      } catch(error) {
        e.ports[0].postMessage(error.name);
      }
    } else if (e.data.cache != undefined) {
      let path = e.data.cache;
      try {
        while (path.length != 0) {
          cache = cache[path.shift()];
        }
        e.ports[0].postMessage('cached');
      } catch (error) {
        e.ports[0].postMessage(error.name);
      }
    } else if (e.data == 'touch-cached') {
      try {
        e.ports[0].postMessage(cache.foo);
      } catch (error) {
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
</script>
<input id="sekrit" value="omg!">
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
  "source_name": "html/browsers/origin/relaxing-the-same-origin-restriction/support/document_domain_frame.html"
}
```
