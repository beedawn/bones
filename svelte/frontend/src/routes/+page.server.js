export function load({cookies}) {

    const bones = cookies.get('bones');
   
        return {
       bones: bones
        };
    }