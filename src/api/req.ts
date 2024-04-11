import {Message} from 'google-protobuf'
export default class Request{
    private readonly baseUrl: string;
    public constructor(baseUrl: string){
        this.baseUrl = baseUrl;
    }
    async Post<T extends Message>(url:string,data:T){
        const res = await fetch(this.baseUrl+url,{
            method: "POST",
            body: data.serialize()
        })
        return await res.arrayBuffer()
    }
    Group(url:string){
        return new Request(this.baseUrl+url)
    }

}
