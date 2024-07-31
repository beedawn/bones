export function load({cookies, params}) {

    const bones = cookies.get('bones');
   
        return {
       bones: bones,
       params: params.slug
        };
    }