# html/semantics/scripting-1/the-script-element/module/resources/credentials-iframe.sub.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/resources/credentials-iframe.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">

<script type="module"
        src="check-cookie.py?id=sameOriginNone&cookieName=same&origin=http://{{host}}:{{ports[http][0]}}">
</script>
<script type="module"
        src="check-cookie.py?id=sameOriginAnonymous&cookieName=same&origin=http://{{host}}:{{ports[http][0]}}"
        crossOrigin="anonymous">
</script>
<script type="module"
        src="check-cookie.py?id=sameOriginUseCredentials&cookieName=same&origin=http://{{host}}:{{ports[http][0]}}"
        crossOrigin="use-credentials">
</script>
<script type="module"
        src="http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/check-cookie.py?id=crossOriginNone&cookieName=cross&origin=http://{{host}}:{{ports[http][0]}}">
</script>
<script type="module"
        src="http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/check-cookie.py?id=crossOriginAnonymous&cookieName=cross&origin=http://{{host}}:{{ports[http][0]}}"
        crossOrigin="anonymous">
</script>
<script type="module"
        src="http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/check-cookie.py?id=crossOriginUseCredentials&cookieName=cross&origin=http://{{host}}:{{ports[http][0]}}"
        crossOrigin="use-credentials">
</script>

<script type="module">
import "./check-cookie.py?id=sameOriginNoneDescendant&cookieName=same&origin=http://{{host}}:{{ports[http][0]}}";
</script>
<script type="module" crossOrigin="anonymous">
import "./check-cookie.py?id=sameOriginAnonymousDescendant&cookieName=same&origin=http://{{host}}:{{ports[http][0]}}";
</script>
<script type="module" crossOrigin="use-credentials">
import "./check-cookie.py?id=sameOriginUseCredentialsDescendant&cookieName=same&origin=http://{{host}}:{{ports[http][0]}}";
</script>
<script type="module">
import "http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/check-cookie.py?id=crossOriginNoneDescendant&cookieName=cross&origin=http://{{host}}:{{ports[http][0]}}";
</script>
<script type="module" crossOrigin="anonymous">
import "http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/check-cookie.py?id=crossOriginAnonymousDescendant&cookieName=cross&origin=http://{{host}}:{{ports[http][0]}}";
</script>
<script type="module" crossOrigin="use-credentials">
import "http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/check-cookie.py?id=crossOriginUseCredentialsDescendant&cookieName=cross&origin=http://{{host}}:{{ports[http][0]}}";
</script>

<script type="text/javascript">
window.addEventListener('load', event => {
  window.parent.postMessage({}, '*');
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.src.invalid",
      "message": "Bad value “http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/check-cookie.py?id=crossOriginNone&cookieName=cross&origin=http://{{host}}:{{ports[http][0]}}” for attribute “src” on element “script”.",
      "severity": "Warning",
      "span": {
        "byte_end": 779,
        "byte_start": 542,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.script.src.invalid",
      "message": "Bad value “http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/check-cookie.py?id=crossOriginAnonymous&cookieName=cross&origin=http://{{host}}:{{ports[http][0]}}” for attribute “src” on element “script”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1064,
        "byte_start": 790,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.script.src.invalid",
      "message": "Bad value “http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/check-cookie.py?id=crossOriginUseCredentials&cookieName=cross&origin=http://{{host}}:{{ports[http][0]}}” for attribute “src” on element “script”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1360,
        "byte_start": 1075,
        "col": 1,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 2745,
        "byte_start": 2714,
        "col": 1,
        "line": 46
      }
    },
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/resources/credentials-iframe.sub.html"
}
```
