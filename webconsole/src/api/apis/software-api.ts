/* tslint:disable */
/* eslint-disable */
/**
 * sctgdesk-api-server
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 0.1.0
 * 
 *
 * NOTE: This class is auto generated by the swagger code generator program.
 * https://github.com/swagger-api/swagger-codegen.git
 * Do not edit the class manually.
 */

import globalAxios, { AxiosResponse, AxiosInstance, AxiosRequestConfig } from 'axios';
import { Configuration } from '../configuration';
// Some imports not used depending on template conditions
// @ts-ignore
import { BASE_PATH, COLLECTION_FORMATS, RequestArgs, BaseAPI, RequiredError } from '../base';
import { SoftwareResponse } from '../models';
import { SoftwareVersionResponse } from '../models';
/**
 * SoftwareApi - axios parameter creator
 * @export
 */
export const SoftwareApiAxiosParamCreator = function (configuration?: Configuration) {
    return {
        /**
         * Get the software download url  # Arguments  * `key` - The key to the software download link, it can be `osx`, `w64` or `ios`  # Usage  * it needs a valid S3 configuration file defined with the `S3_CONFIG_FILE` environment variable  <pre> [s3config] Endpoint = \"https://compat.objectstorage.eu-london-1.oraclecloud.com\" Region = \"eu-london-1\" AccessKey = \"c324ead11faa0d87337c07ddc4a1129fab76188d\" SecretKey = \"GJurV55f/LD36kjZFpchZMj/uvgTqxHyFkBchUUa8KA=\" Bucket = \"aezoz24elapn\" Windows64Key = \"master/sctgdesk-releases/sctgdesk-1.2.4-x86_64.exe\" Windows32Key = \"master/sctgdesk-releases/sctgdesk-1.2.4-i686.exe\" OSXKey = \"master/sctgdesk-releases/sctgdesk-1.2.4.dmg\" OSXArm64Key = \"master/sctgdesk-releases/sctgdesk-1.2.4.dmg\" IOSKey = \"master/sctgdesk-releases/sctgdesk-1.2.4.ipa\" </pre>
         * @param {string} key 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        software: async (key: string, options: AxiosRequestConfig = {}): Promise<RequestArgs> => {
            // verify required parameter 'key' is not null or undefined
            if (key === null || key === undefined) {
                throw new RequiredError('key','Required parameter key was null or undefined when calling software.');
            }
            const localVarPath = `/api/software/client-download-link/{key}`
                .replace(`{${"key"}}`, encodeURIComponent(String(key)));
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, 'https://example.com');
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }
            const localVarRequestOptions :AxiosRequestConfig = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;

            const query = new URLSearchParams(localVarUrlObj.search);
            for (const key in localVarQueryParameter) {
                query.set(key, localVarQueryParameter[key]);
            }
            for (const key in options.params) {
                query.set(key, options.params[key]);
            }
            localVarUrlObj.search = (new URLSearchParams(query)).toString();
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: localVarUrlObj.pathname + localVarUrlObj.search + localVarUrlObj.hash,
                options: localVarRequestOptions,
            };
        },
        /**
         * Retrieve the server version
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        softwareVersion: async (options: AxiosRequestConfig = {}): Promise<RequestArgs> => {
            const localVarPath = `/api/software/version/server`;
            // use dummy base URL string because the URL constructor only accepts absolute URLs.
            const localVarUrlObj = new URL(localVarPath, 'https://example.com');
            let baseOptions;
            if (configuration) {
                baseOptions = configuration.baseOptions;
            }
            const localVarRequestOptions :AxiosRequestConfig = { method: 'GET', ...baseOptions, ...options};
            const localVarHeaderParameter = {} as any;
            const localVarQueryParameter = {} as any;

            const query = new URLSearchParams(localVarUrlObj.search);
            for (const key in localVarQueryParameter) {
                query.set(key, localVarQueryParameter[key]);
            }
            for (const key in options.params) {
                query.set(key, options.params[key]);
            }
            localVarUrlObj.search = (new URLSearchParams(query)).toString();
            let headersFromBaseOptions = baseOptions && baseOptions.headers ? baseOptions.headers : {};
            localVarRequestOptions.headers = {...localVarHeaderParameter, ...headersFromBaseOptions, ...options.headers};

            return {
                url: localVarUrlObj.pathname + localVarUrlObj.search + localVarUrlObj.hash,
                options: localVarRequestOptions,
            };
        },
    }
};

/**
 * SoftwareApi - functional programming interface
 * @export
 */
export const SoftwareApiFp = function(configuration?: Configuration) {
    return {
        /**
         * Get the software download url  # Arguments  * `key` - The key to the software download link, it can be `osx`, `w64` or `ios`  # Usage  * it needs a valid S3 configuration file defined with the `S3_CONFIG_FILE` environment variable  <pre> [s3config] Endpoint = \"https://compat.objectstorage.eu-london-1.oraclecloud.com\" Region = \"eu-london-1\" AccessKey = \"c324ead11faa0d87337c07ddc4a1129fab76188d\" SecretKey = \"GJurV55f/LD36kjZFpchZMj/uvgTqxHyFkBchUUa8KA=\" Bucket = \"aezoz24elapn\" Windows64Key = \"master/sctgdesk-releases/sctgdesk-1.2.4-x86_64.exe\" Windows32Key = \"master/sctgdesk-releases/sctgdesk-1.2.4-i686.exe\" OSXKey = \"master/sctgdesk-releases/sctgdesk-1.2.4.dmg\" OSXArm64Key = \"master/sctgdesk-releases/sctgdesk-1.2.4.dmg\" IOSKey = \"master/sctgdesk-releases/sctgdesk-1.2.4.ipa\" </pre>
         * @param {string} key 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async software(key: string, options?: AxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => Promise<AxiosResponse<SoftwareResponse>>> {
            const localVarAxiosArgs = await SoftwareApiAxiosParamCreator(configuration).software(key, options);
            return (axios: AxiosInstance = globalAxios, basePath: string = BASE_PATH) => {
                const axiosRequestArgs :AxiosRequestConfig = {...localVarAxiosArgs.options, url: basePath + localVarAxiosArgs.url};
                return axios.request(axiosRequestArgs);
            };
        },
        /**
         * Retrieve the server version
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async softwareVersion(options?: AxiosRequestConfig): Promise<(axios?: AxiosInstance, basePath?: string) => Promise<AxiosResponse<SoftwareVersionResponse>>> {
            const localVarAxiosArgs = await SoftwareApiAxiosParamCreator(configuration).softwareVersion(options);
            return (axios: AxiosInstance = globalAxios, basePath: string = BASE_PATH) => {
                const axiosRequestArgs :AxiosRequestConfig = {...localVarAxiosArgs.options, url: basePath + localVarAxiosArgs.url};
                return axios.request(axiosRequestArgs);
            };
        },
    }
};

/**
 * SoftwareApi - factory interface
 * @export
 */
export const SoftwareApiFactory = function (configuration?: Configuration, basePath?: string, axios?: AxiosInstance) {
    return {
        /**
         * Get the software download url  # Arguments  * `key` - The key to the software download link, it can be `osx`, `w64` or `ios`  # Usage  * it needs a valid S3 configuration file defined with the `S3_CONFIG_FILE` environment variable  <pre> [s3config] Endpoint = \"https://compat.objectstorage.eu-london-1.oraclecloud.com\" Region = \"eu-london-1\" AccessKey = \"c324ead11faa0d87337c07ddc4a1129fab76188d\" SecretKey = \"GJurV55f/LD36kjZFpchZMj/uvgTqxHyFkBchUUa8KA=\" Bucket = \"aezoz24elapn\" Windows64Key = \"master/sctgdesk-releases/sctgdesk-1.2.4-x86_64.exe\" Windows32Key = \"master/sctgdesk-releases/sctgdesk-1.2.4-i686.exe\" OSXKey = \"master/sctgdesk-releases/sctgdesk-1.2.4.dmg\" OSXArm64Key = \"master/sctgdesk-releases/sctgdesk-1.2.4.dmg\" IOSKey = \"master/sctgdesk-releases/sctgdesk-1.2.4.ipa\" </pre>
         * @param {string} key 
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async software(key: string, options?: AxiosRequestConfig): Promise<AxiosResponse<SoftwareResponse>> {
            return SoftwareApiFp(configuration).software(key, options).then((request) => request(axios, basePath));
        },
        /**
         * Retrieve the server version
         * @param {*} [options] Override http request option.
         * @throws {RequiredError}
         */
        async softwareVersion(options?: AxiosRequestConfig): Promise<AxiosResponse<SoftwareVersionResponse>> {
            return SoftwareApiFp(configuration).softwareVersion(options).then((request) => request(axios, basePath));
        },
    };
};

/**
 * SoftwareApi - object-oriented interface
 * @export
 * @class SoftwareApi
 * @extends {BaseAPI}
 */
export class SoftwareApi extends BaseAPI {
    /**
     * Get the software download url  # Arguments  * `key` - The key to the software download link, it can be `osx`, `w64` or `ios`  # Usage  * it needs a valid S3 configuration file defined with the `S3_CONFIG_FILE` environment variable  <pre> [s3config] Endpoint = \"https://compat.objectstorage.eu-london-1.oraclecloud.com\" Region = \"eu-london-1\" AccessKey = \"c324ead11faa0d87337c07ddc4a1129fab76188d\" SecretKey = \"GJurV55f/LD36kjZFpchZMj/uvgTqxHyFkBchUUa8KA=\" Bucket = \"aezoz24elapn\" Windows64Key = \"master/sctgdesk-releases/sctgdesk-1.2.4-x86_64.exe\" Windows32Key = \"master/sctgdesk-releases/sctgdesk-1.2.4-i686.exe\" OSXKey = \"master/sctgdesk-releases/sctgdesk-1.2.4.dmg\" OSXArm64Key = \"master/sctgdesk-releases/sctgdesk-1.2.4.dmg\" IOSKey = \"master/sctgdesk-releases/sctgdesk-1.2.4.ipa\" </pre>
     * @param {string} key 
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof SoftwareApi
     */
    public async software(key: string, options?: AxiosRequestConfig) : Promise<AxiosResponse<SoftwareResponse>> {
        return SoftwareApiFp(this.configuration).software(key, options).then((request) => request(this.axios, this.basePath));
    }
    /**
     * Retrieve the server version
     * @param {*} [options] Override http request option.
     * @throws {RequiredError}
     * @memberof SoftwareApi
     */
    public async softwareVersion(options?: AxiosRequestConfig) : Promise<AxiosResponse<SoftwareVersionResponse>> {
        return SoftwareApiFp(this.configuration).softwareVersion(options).then((request) => request(this.axios, this.basePath));
    }
}
