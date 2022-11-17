document.querySelectorAll('.location').forEach(
    (element) => {
        element.innerHTML = location.href;
        element.href = location.href;
    }
)

let crt_location = location.href + 'certificate.crt';
let crt_element = document.querySelector('.location-crt');

crt_element.innerHTML = crt_location;
crt_element.href = crt_location;
